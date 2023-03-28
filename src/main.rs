use std::path::PathBuf;

use anyhow::anyhow;
use clap::Parser;
use limg::Arguments;

pub fn convert(mut path: PathBuf, format: &str) -> Result<(), anyhow::Error> {
    let image = image::io::Reader::open(path.clone())?.decode()?;

    if !path.set_extension(format) {
        return Err(anyhow!("Could not change extension for: {:?}", path));
    }

    image.save(path)?;

    Ok(())
}

fn main() {
    let args = Arguments::parse();

    for image in glob::glob(&args.input).expect("invalid glob pattern") {
        match image {
            Ok(path) => {
                match convert(path, &args.target_format) {
                    Ok(_) => {},
                    Err(err) => println!("{err}"),
                }
            },
            Err(e) => println!("Failed to convert with error: {e:?}"),
        }
    }
}
