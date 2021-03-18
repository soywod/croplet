use clap::{self, Arg};
use error_chain::error_chain;
use std::{env, path::PathBuf};

use crate::dir;
use crate::img;

error_chain! {
    links {
        Dir(dir::Error, dir::ErrorKind);
        Img(img::Error, img::ErrorKind);
    }
}

pub struct App<'a>(pub clap::App<'a, 'a>);

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self(
            clap::App::new(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
                .about(env!("CARGO_PKG_DESCRIPTION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .setting(clap::AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("input")
                        .long("input")
                        .short("i")
                        .help("Source directory containing raw booklet scanned pages")
                        .value_name("DIR")
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .long("output")
                        .short("o")
                        .help("Target directory to store processed pages")
                        .value_name("DIR")
                        .required(true),
                )
                .arg(
                    Arg::with_name("parts")
                        .long("parts")
                        .short("p")
                        .help("Number of sub-booklet the booklet is composed of")
                        .value_name("INT")
                        .default_value("1"),
                )
                .arg(
                    Arg::with_name("reverse")
                        .long("reverse")
                        .short("r")
                        .help("Reverses the order"),
                ),
        )
    }

    pub fn run(self) -> Result<()> {
        let matches = self.0.get_matches();
        let input: PathBuf = matches.value_of("input").unwrap().parse().unwrap();
        let output: PathBuf = matches.value_of("output").unwrap().parse().unwrap();
        // let parts: u8 = matches.value_of("parts").unwrap().parse().unwrap();

        let paths = dir::get_picture_paths(&input)?;
        img::process(&output, &paths)?;

        Ok(())
    }
}
