mod color;

use clap::Parser;
use std::{fs, path::Path};

#[derive(Debug, Clone, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Args {
    #[clap(help = "Symbolic link to be delete")]
    filename: String,
}

fn main() {
    let Args { filename } = Args::parse();
    let path = Path::new(&filename);

    if !path.exists() {
        println!(
            "{} The specified path {} does not exist",
            color::error(),
            color::identifier(&filename)
        );

        return;
    }

    let metadata = match fs::symlink_metadata(&filename) {
        Ok(metadata) => metadata,
        Err(_) => {
            println!("{} Unable to retrieve metadata", color::error());

            return;
        }
    };
    let file_type = metadata.file_type();

    if !file_type.is_symlink() {
        println!(
            "{} The specified name is not a symbolic link\n\n {} A slash at the end of a symbolic link name will give a different meaning",
            color::error(),
            color::note()
        );

        return;
    }

    match fs::remove_file(&filename) {
        Ok(_) => {
            println!(
                "{} Symbolic link {} removed",
                color::successfully(),
                color::identifier(&filename)
            );
        }
        Err(_) => {
            println!(
                "{} Failed to remove symbolic link {}",
                color::error(),
                color::identifier(&filename)
            );
        }
    };
}
