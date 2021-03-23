use clap::{self, value_t, Arg};
use error_chain::error_chain;
use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::*;
use std::{env, path::PathBuf, result};

use crate::dir;
use crate::img;

error_chain! {
    foreign_links {
        Clap(clap::Error);
    }
    links {
        Dir(dir::Error, dir::ErrorKind);
        Img(img::Error, img::ErrorKind);
    }
}

pub struct App<'a>(pub clap::App<'a, 'a>);

impl<'a> App<'a> {
    fn margin_validator(s: String) -> result::Result<(), String> {
        s.parse::<u32>().map_err(|_| "should be > 0")?;
        Ok(())
    }

    fn parts_validator(s: String) -> result::Result<(), String> {
        let parts_len: usize = s.parse().map_err(|_| "should be > 0")?;
        if parts_len < 1 {
            Err(String::from("should be > 0"))
        } else {
            Ok(())
        }
    }

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
                    Arg::with_name("margin")
                        .long("margin")
                        .short("m")
                        .help("Amount of pixels to remove around pictures")
                        .value_name("INT")
                        .default_value("0")
                        .validator(Self::margin_validator),
                )
                .arg(
                    Arg::with_name("parts")
                        .long("parts")
                        .short("p")
                        .help("Number of sub-booklet the main booklet is composed of")
                        .value_name("INT")
                        .default_value("1")
                        .validator(Self::parts_validator),
                ),
        )
    }

    pub fn run(self) -> Result<()> {
        let matches = self.0.get_matches();
        let input: PathBuf = matches.value_of("input").unwrap().parse().unwrap();
        let output: PathBuf = matches.value_of("output").unwrap().parse().unwrap();
        let margin = value_t!(matches.value_of("margin"), u32)?;
        let parts_len = value_t!(matches.value_of("parts"), usize)?;

        let paths = dir::get_picture_paths(&input)?;
        let paths_len = paths.len();

        if paths_len % 2 != 0 {
            return Err(format!(
                "The number of pictures must be a multiple of 2 (got {})",
                paths_len
            )
            .into());
        };

        if paths_len % parts_len != 0 {
            return Err(format!(
                "The number of pictures must be a multiple of {} (got {})",
                parts_len, paths_len
            )
            .into());
        };

        let chunks = paths.chunks(paths_len / parts_len).collect::<Vec<_>>();
        let zero_fill_len = chunks.len() / 10;
        let mut errs = vec![];

        for (idx, &chunk) in chunks.iter().enumerate() {
            let prefix = format!("{:0fill$}", idx + 1, fill = zero_fill_len);
            errs.extend(
                chunk
                    .par_iter()
                    .enumerate()
                    .progress_count(chunk.len() as u64)
                    .map(|(idx, path)| {
                        let zero_fill_len = chunk.len() / 10;
                        img::process(
                            &output,
                            margin,
                            &prefix,
                            idx,
                            &path,
                            paths_len * 2,
                            zero_fill_len,
                        )
                        .chain_err(|| {
                            format!("Could not process picture `{}`", path.to_string_lossy())
                        })
                    })
                    .filter_map(|res| res.err())
                    .collect::<Vec<_>>(),
            );
        }

        if !errs.is_empty() {
            errs.iter().for_each(|ref errs| {
                let mut errs = errs.iter();
                match errs.next() {
                    None => (),
                    Some(err) => {
                        eprintln!("{}", err);
                        errs.for_each(|err| eprintln!(" ↳ {}", err));
                    }
                }
            })
        };

        Ok(())
    }
}
