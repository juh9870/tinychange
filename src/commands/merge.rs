use crate::config::CommandOpts;
use crate::tinychange::TinyChange;
use clap::Args;
use miette::{bail, Context, IntoDiagnostic};
use regex::{Regex, RegexBuilder};
use std::borrow::Cow;
use std::ops::Range;

#[derive(Debug, Default, Clone, Args)]
pub struct MergeArgs {
    /// Do not delete tinychange files after merging them into the changelog
    #[arg(short, long)]
    keep: bool,
}

fn regex_for_section(section: &str) -> Regex {
    RegexBuilder::new(&format!(r"^#+\s*\[?\s*{}\s*]?[^\n]*$", section))
        .case_insensitive(true)
        .build()
        .unwrap()
}
impl MergeArgs {
    pub fn run(self, opts: CommandOpts) -> miette::Result<()> {
        let mut all_changes = vec![];
        let mut to_delete = vec![];

        for file in fs_err::read_dir(opts.tinychanges_dir()).into_diagnostic()? {
            let file = file.into_diagnostic()?;
            if file.path().is_dir() {
                bail!(
                    "Unexpected directory found in tinychanges directory: {:?}",
                    file.path()
                );
            }

            if file.path().extension() != Some("md".as_ref()) {
                continue;
            }

            let content = fs_err::read_to_string(file.path()).into_diagnostic()?;
            let change = TinyChange::deserialize(&opts, content).with_context(|| {
                format!(
                    "Failed to deserialize tinychange at {}",
                    file.path().display()
                )
            })?;
            to_delete.push(file.path());
            all_changes.push(change);
        }

        if all_changes.is_empty() {
            opts.println("No tinychanges found, nothing to do");
            return Ok(());
        }

        if !opts.changelog_file().exists() {
            opts.println("No changelog file found, creating a new one");
            let content = format_changesets(&opts, all_changes, None)?;
            let content = format!("# Changelog\n\n## [Unreleased]\n{}", content);
            fs_err::write(opts.changelog_file(), content).into_diagnostic()?;
        } else {
            let old_content = fs_err::read_to_string(opts.changelog_file()).into_diagnostic()?;

            let mut lines = old_content.lines().map(Cow::Borrowed).collect::<Vec<_>>();

            if let Some(unreleased_section) = find_section(
                &lines,
                0..lines.len(),
                false,
                &regex_for_section("unreleased"),
            ) {
                opts.println("Found unreleased section, merging changes into it");
                let mut existing_sections = vec![];
                let mut existing_section_ranges = vec![];
                for category in opts.categories() {
                    if let Some(section) = find_section(
                        &lines,
                        unreleased_section.clone(),
                        false,
                        &regex_for_section(category),
                    ) {
                        existing_section_ranges.push(section.clone());
                        // skip the first line, which is the section header
                        existing_sections
                            .push(Some(lines[(section.start + 1)..section.end].join("\n")));
                    } else {
                        existing_sections.push(None);
                    }
                }

                let earliest_section_start = existing_section_ranges
                    .iter()
                    .map(|r| r.start)
                    .min()
                    .unwrap_or(unreleased_section.end);
                let latest_section_end = existing_section_ranges
                    .iter()
                    .map(|r| r.end)
                    .max()
                    .unwrap_or(unreleased_section.end);

                if !existing_section_ranges.is_empty() {
                    #[allow(clippy::needless_range_loop)]
                    for idx in earliest_section_start..latest_section_end {
                        if lines[idx].trim().is_empty() {
                            continue;
                        }

                        if !existing_section_ranges
                            .iter()
                            .any(|range| range.contains(&idx))
                        {
                            bail!("Unexpected content or unknown category in unreleased section at line {}: {}", idx + 1, lines[idx]);
                        }
                    }
                }

                let content = format_changesets(&opts, all_changes, Some(existing_sections))?;

                let mut cutoff_start = earliest_section_start;
                while cutoff_start > 0 && lines[cutoff_start - 1].trim().is_empty() {
                    cutoff_start -= 1;
                }

                // discard the full unreleased section except for the header
                let before = &lines[..cutoff_start];
                let after = &lines[latest_section_end..];

                lines = before
                    .iter()
                    .cloned()
                    .chain([Cow::Owned(content)])
                    .chain(after.iter().cloned())
                    .collect();
            } else if let Some(changelog_section) = find_section(
                &lines,
                0..lines.len(),
                true,
                &regex_for_section("changelog"),
            ) {
                let content = format_changesets(&opts, all_changes, None)?;
                opts.println(
                    "No unreleased section found, creating a new one under the changelog section",
                );

                let place = changelog_section.end;

                lines.insert(place, Cow::Owned(content));
                lines.insert(place, "\n## [Unreleased]".into());
                if place > 0 && lines[place - 1].trim().is_empty() {
                    lines.remove(place - 1);
                }
            } else {
                bail!("No unreleased or changelog section found in changelog file")
            };

            fs_err::write(opts.changelog_file(), lines.join("\n")).into_diagnostic()?;
        };

        if !self.keep {
            for file in to_delete {
                fs_err::remove_file(file).into_diagnostic()?;
            }
        }

        Ok(())
    }
}

/// Finds the section in the changelog file that starts with the given regex
/// and ends with the next section.
///
/// Returns the range of lines that the section spans
fn find_section(
    lines: &[Cow<str>],
    search_in: Range<usize>,
    ignore_level_matching: bool,
    section_start: &Regex,
) -> Option<Range<usize>> {
    let mut start = None::<(usize, i32)>;
    for (idx, line) in lines[search_in.clone()]
        .iter()
        .enumerate()
        .filter(|l| l.1.starts_with('#'))
    {
        let idx = idx + search_in.start;
        let level = line.chars().take_while(|c| *c == '#').count() as i32;
        if let Some((start_idx, start_level)) = &start {
            if level <= *start_level || ignore_level_matching {
                return Some(*start_idx..idx);
            }
        } else if section_start.is_match(line) {
            start = Some((idx, level));
        }
    }
    if let Some((start_idx, _)) = start {
        Some(start_idx..search_in.end)
    } else {
        None
    }
}

fn format_changesets(
    opts: &CommandOpts,
    all_changes: Vec<TinyChange>,
    existing_sections: Option<Vec<Option<String>>>,
) -> miette::Result<String> {
    opts.println(&format!("Merging {} changesets", all_changes.len()));

    let mut builder = String::new();

    for (idx, category) in opts.categories().iter().enumerate() {
        let existing = existing_sections
            .as_ref()
            .and_then(|sections| sections[idx].as_ref());

        let mut changes = all_changes
            .iter()
            .filter(|change| &change.kind == category)
            .peekable();

        if changes.peek().is_none() && existing.is_none() {
            continue;
        }

        builder.push_str(&format!("\n### {}\n\n", category));
        if let Some(existing) = existing {
            builder.push_str(existing.trim());
            builder.push('\n');
        }

        for change in changes {
            builder.push_str(&change.as_markdown().to_string());
            builder.push('\n');
        }
    }

    Ok(builder)
}
