use clap::Parser;
use std::{
    fs::{read_dir, rename},
    io::Error,
    path::PathBuf,
    process,
};
use unicode_normalization::UnicodeNormalization;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, default_value = "nfc")]
    form: String,
    #[arg(short, default_value_t = false)]
    recursive: bool,
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let form = match args.form.as_str() {
        "nfd" => |s: &str| s.nfd().collect::<String>(),
        "nfc" => |s: &str| s.nfc().collect::<String>(),
        "nfkd" => |s: &str| s.nfkd().collect::<String>(),
        "nfkc" => |s: &str| s.nfkc().collect::<String>(),
        e => {
            eprintln!("{} is an undefined form.", e);
            process::exit(1);
        }
    };

    if !args.recursive {
        if let Some(path) = args.paths.iter().find(|&p| p.is_dir()) {
            eprintln!("{} is directory.", path.to_str().unwrap());
            process::exit(1);
        }
    }

    for path in args.paths {
        normalize(path, args.recursive, form)?;
    }

    return Ok(());
}

fn normalize(path: PathBuf, recursive: bool, form: fn(&str) -> String) -> Result<(), Error> {
    if recursive && path.is_dir() {
        if let Ok(dir) = read_dir(&path) {
            for result in dir {
                normalize(result?.path(), recursive, form)?;
            }
        }
    }

    let name = path.file_name().and_then(|n| n.to_str()).unwrap();
    let normalized: String = form(name);
    if !normalized.eq(name) {
        let mut to = path.parent().map(|p| p.to_path_buf()).unwrap();
        to.push(&normalized);
        println!(
            "{}: {} -> {}",
            path.to_str().unwrap(),
            normalized.len(),
            to.to_str().unwrap().len()
        );
        rename(path, to)?;
    }

    return Ok(());
}
