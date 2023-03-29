use std::{path::{Path, PathBuf}, fs};

use anyhow::anyhow;
use clap::Parser;
use limg::Arguments;

pub fn convert<Q: AsRef<Path>>(path: Q, format: &str) -> Result<PathBuf, anyhow::Error> {
    let mut p = path.as_ref().to_path_buf();

    let image = image::io::Reader::open(&p)?.decode()?;

    if !p.set_extension(format) {
        return Err(anyhow!("Could not change extension for: {:?}", p));
    }

    image.save(&p)?;

    Ok(p)
}

fn main() {
    let args = Arguments::parse();

    for image in glob::glob(&args.input).expect("Invalid glob pattern") {
        match image {
            Ok(path) => {
                match convert(&path, &args.target_format) {
                    Ok(output) => {
                        println!("{path:?} -> {output:?}");
                        if args.delete_after {
                            if let Err(e) = fs::remove_file(&path) {
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
