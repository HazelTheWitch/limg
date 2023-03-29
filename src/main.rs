use std::{path::{Path, PathBuf}, fs};

use anyhow::anyhow;
use clap::Parser;
use image::ImageFormat;
use limg::Arguments;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub fn convert<Q: AsRef<Path>>(path: Q, format: &str) -> Result<PathBuf, anyhow::Error> {
    let mut p = path.as_ref().to_path_buf();

    let image = image::io::Reader::open(&p)?.decode()?;

    if !p.set_extension(format) {
        return Err(anyhow!("Could not change extension for: {p:?}"));
    }

    image.save(&p)?;

    Ok(p)
}

fn main() -> Result<(), anyhow::Error> {
    let args = Arguments::parse();

    if ImageFormat::from_extension(&args.target_format) == None {
        return Err(anyhow!("Could not determine format from extension: {:?}", args.target_format));
    }

    glob::glob(&args.input)?
        .collect::<Vec<Result<PathBuf, _>>>()
        .into_par_iter()
        .for_each(|image| {
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
        });

    Ok(())
}
