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
    #[cfg(windows)]
    unsafe {
        use winapi::um::{wincon::SetConsoleOutputCP, winnls::CP_UTF8};
        SetConsoleOutputCP(CP_UTF8);
    }

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

    for path in args.paths {
        if !path.exists() {
            eprintln!("{} is not exists.", path.to_str().unwrap());
            process::exit(1);
        }
        if !args.recursive && path.is_dir() {
            eprintln!("{} is directory.", path.to_str().unwrap());
            process::exit(1);
        }
        normalize(path, args.recursive, form)?;
    }

    return Ok(());
}

fn normalize(path: PathBuf, recursive: bool, form: fn(&str) -> String) -> Result<(), Error> {
    if recursive && path.is_dir() {
        match read_dir(&path) {
            Ok(dir) => {
                for result in dir {
                    normalize(result?.path(), recursive, form)?;
                }
            }
            Err(e) => eprintln!("{} - {}", path.to_str().unwrap(), e),
        }
    }

    let name = path.file_name().and_then(|n| n.to_str()).unwrap();
    let normalized: String = form(name);
    if !normalized.eq(name) {
        let mut to = path.parent().map(|p| p.to_path_buf()).unwrap();
        to.push(&normalized);
        println!("{}", path.to_str().unwrap());
        rename(path, to)?;
    }

    return Ok(());
}
