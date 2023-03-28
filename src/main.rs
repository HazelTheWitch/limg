use std::{path::PathBuf, fs};

use anyhow::anyhow;
use clap::Parser;
use limg::Arguments;

pub fn convert(mut path: PathBuf, format: &str) -> Result<(), anyhow::Error> {
    let image = image::io::Reader::open(path.clone())?.decode()?;

    if !path.set_extension(format) {
        return Err(anyhow!("Could not change extension for: {path:?}"));
    }

    image.save(path)?;

    Ok(())
}

fn main() {
    let args = Arguments::parse();

    for image in glob::glob(&args.input).expect("Invalid glob pattern") {
        match image {
            Ok(path) => {
                match convert(path.clone(), &args.target_format) {
                    Ok(_) => {
                        if args.delete_after {
                            if let Err(e) = fs::remove_file(path.clone()) {
                                println!("Error removing {path:?}: {e:?}");
                            }
                        }
                    },
                    Err(err) => println!("{path:?}: {err}"),
                }
            },
            Err(e) => println!("Failed to read with error: {e:?}"),
        }
    }
}
