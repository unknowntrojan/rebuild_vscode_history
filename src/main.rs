#![feature(string_remove_matches)]

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    path: Option<String>,
    #[arg(long)]
    prefix: Option<String>,
}

#[derive(serde::Deserialize, Clone)]
struct Entry {
    pub id: String,
    pub timestamp: u64,
}

#[derive(serde::Deserialize, Clone)]
struct EntriesJson {
    #[serde(rename = "version")]
    pub _version: u64,
    pub resource: String,
    pub entries: Vec<Entry>,
}

impl EntriesJson {
    fn find_newest_entry(&self) -> Entry {
        let mut newest_entry = Entry {
            id: "".into(),
            timestamp: 0,
        };

        for entry in &self.entries {
            if entry.timestamp > newest_entry.timestamp {
                newest_entry = entry.clone();
            }
        }

        newest_entry
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let _ = std::fs::create_dir("out");

    let path = match args.path {
        Some(x) => x,
        None => format!(
            "{}\\Code\\User\\History",
            std::env::var("APPDATA").expect("must specify history folder path on non-windows OS!")
        ),
    };

    for entry in std::fs::read_dir(&path)? {
        if let Ok(entry) = entry {
            let entries =
                std::fs::read(format!("{}/entries.json", entry.path().to_str().unwrap()))?;

            let entries: EntriesJson = serde_json::from_slice(&entries)?;

            let newest_entry = entries.find_newest_entry();

            // avert your eyes, kids
            let fixed_path = if let Some(prefix) = &args.prefix {
                let (_, fixed_path) = match entries.resource.split_once(prefix) {
                    Some(x) => x,
                    None => {
                        println!("wtf? {}", entries.resource);
                        continue;
                    }
                };
                fixed_path.to_string()
            } else {
                let (_, path) = match entries.resource.split_once("file:///") {
                    Some(x) => x,
                    None => {
                        println!("wtf? {}", entries.resource);
                        continue;
                    }
                };

                let mut path = url_escape::decode(path).to_string();
                path.remove_matches(":");
                path
            };

            let fixed_path = format!("out/{}", fixed_path);

            // create folders
            let (path_without_file, _) = &fixed_path.rsplit_once("/").unwrap();
            std::fs::create_dir_all(path_without_file)?;

            std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), newest_entry.id),
                fixed_path,
            )?;
        }
    }

    Ok(())
}
