use fake::{faker, Fake};
use miette::bail;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NameType {
    #[default]
    Buzzword,
    Lorem,
    Hash,
}

const MAX_TRIES: usize = 100;

impl NameType {
    pub fn generate(&self, seed: u64, max_length: Option<usize>) -> miette::Result<String> {
        let max_length = max_length.unwrap_or(127);
        let mut rng = &mut SmallRng::seed_from_u64(seed);
        let mut tries = 0;
        loop {
            tries += 1;
            let hash = format!("{:x}", seed);
            let short_hash = &hash[..7];
            let name = match self {
                NameType::Buzzword => {
                    let start: String = faker::company::en::Buzzword().fake_with_rng(&mut rng);
                    let middle: String =
                        faker::company::en::BuzzwordMiddle().fake_with_rng(&mut rng);
                    let tail: String = faker::company::en::BuzzwordTail().fake_with_rng(&mut rng);

                    format!("{}-{}-{}-{}", start, middle, tail, short_hash)
                        .replace(' ', "-")
                        .to_lowercase()
                }
                NameType::Lorem => {
                    let words: Vec<String> = faker::lorem::en::Words(3..4).fake_with_rng(&mut rng);
                    let word = words.join("-").to_lowercase();
                    format!("{}-{}", word, short_hash)
                }
                NameType::Hash => hash,
            };

            let name = sanitise_file_name::sanitise(&name);

            if name.len() <= max_length {
                return Ok(name);
            }
            if tries > MAX_TRIES {
                bail!(
                    help = "Try increasing the `max_filename_length` in your configuration",
                    "Failed to generate a filename of length {} within {} tries",
                    max_length,
                    MAX_TRIES
                );
            }
        }
    }
}
