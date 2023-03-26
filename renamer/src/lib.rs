use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("renamer")
        .version("0.1.0")
        .about("Rename files")
        .arg(
            Arg::with_name("files")
            .value_name("FILE")
            .short("f")
            .long("filename")
            .help("Input file(s)")
            .multiple(true)
            .default_value("-"),
        )
        .arg(
            Arg::with_name("directories")
            .short("d")
            .long("dir")
            .value_name("DIRECTORY")
            .help("Input directory"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    for filename in config.files{
        println!("{:#?}", filename)
    }
    Ok(())
}