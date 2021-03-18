use error_chain::error_chain;
use image::imageops;
use std::path::{Path, PathBuf};

error_chain! {}

pub fn process(dir: &Path, margin: u32, chunk_prefix: &str, paths: &[PathBuf]) -> Result<()> {
    let pages_count = paths.len() * 2;

    for (idx, path) in paths.iter().enumerate() {
        let filename = path
            .file_name()
            .ok_or_else(|| format!("Invalid filename `{}`", path.to_string_lossy()))?
            .to_string_lossy();

        println!("Processing `{}`:", filename);
        println!(" [Opening]");
        let img = image::open(&path)
            .chain_err(|| format!("Cannot open image `{}`", path.to_string_lossy()))?;

        println!(" [Rotating]");
        let mut img = if idx % 2 == 0 {
            imageops::rotate90(&img)
        } else {
            imageops::rotate270(&img)
        };
        let (width, height) = img.dimensions();

        println!(" [Croping left]");
        let left = imageops::crop(
            &mut img,
            margin,
            margin,
            width / 2 - margin,
            height - margin,
        )
        .to_image();
        let mut left_path = dir.to_path_buf();
        let prefix = format!("{:0fill$}", pages_count - idx, fill = paths.len() / 10);
        left_path.push(format!("{}__{}_{}", chunk_prefix, prefix, filename));
        println!(" [Saving left]");
        left.save(&left_path)
            .chain_err(|| format!("Cannot save left part of `{}`", &filename))?;

        println!(" [Croping right]");
        let right = imageops::crop(
            &mut img,
            width / 2 - margin,
            margin,
            width - margin,
            height - margin,
        )
        .to_image();
        let mut right_path = dir.to_path_buf();
        let prefix = format!("{:0fill$}", idx + 1, fill = paths.len() / 10);
        right_path.push(format!("{}__{}_{}", chunk_prefix, prefix, filename));
        println!(" [Saving right]");
        right
            .save(&right_path)
            .chain_err(|| format!("Cannot save right part of `{}`", &filename))?;
    }

    Ok(())
}
