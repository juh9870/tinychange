use crate::TinyChangeArgs;
use clap::Parser;
use miette::{Context, IntoDiagnostic};
use std::path::Path;

fn run_changelog_test(test_dir: impl AsRef<Path>) -> miette::Result<String> {
    let test_dir = test_dir.as_ref();
    let temp_dir = temp_dir::TempDir::new().expect("Should create a temporary directory");

    std::env::set_current_dir(temp_dir.path()).expect("Should set current working directory");

    dircpy::copy_dir(test_dir, temp_dir.path()).expect("Should copy test files to temp directory");

    let commands = fs_err::read_to_string(temp_dir.path().join("commands.txt"))
        .expect("Should read commands.txt");

    for (idx, command) in commands.lines().enumerate() {
        let args = (|| {
            let words = shell_words::split(command)
                .into_diagnostic()
                .context("Failed to split command into shell words")?;
            let args = TinyChangeArgs::try_parse_from(words)
                .into_diagnostic()
                .context("Failed to parse command line arguments")?;
            Ok(args)
        })()
        .unwrap_or_else(|err: miette::Error| {
            panic!(
                "{:?}",
                err.context(format!(
                    "Failed to parse arguments for command #{}: {}",
                    idx, command
                ))
            )
        });

        crate::run(args, "tinychange")
            .context("Failed to run tinychange")
            .with_context(|| format!("Failed to execute command #{}: {}", idx, command))?;
    }

    let changelog = fs_err::read_to_string(temp_dir.path().join("CHANGELOG.md"))
        .expect("Should be able to read CHANGELOG.md");

    temp_dir.cleanup().expect("Should cleanup temp directory");

    Ok(changelog)
}

#[test]
fn changelog_tests() {
    let cwd = std::env::current_dir().expect("Should get current working directory");
    insta::glob!("cases/*", |path| {
        std::env::set_current_dir(&cwd).expect("Should set current working directory");
        let text = match run_changelog_test(path) {
            Ok(changelog) => {
                format!("Changelog\n---\n{}", changelog)
            }
            Err(err) => {
                format!("Error\n---\n{:?}", err)
            }
        };

        let text = strip_ansi_escapes::strip_str(&text);

        insta::assert_snapshot!(text);
    });
}
