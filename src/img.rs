use error_chain::error_chain;
use image::{imageops, GenericImageView};
use std::path::{Path, PathBuf};

error_chain! {}

pub fn process(dir: &Path, paths: &[PathBuf]) -> Result<()> {
    let pages_count = paths.len() * 2;

    for (idx, path) in paths.iter().enumerate() {
        let filename = path
            .file_name()
            .ok_or_else(|| format!("Invalid filename `{}`", path.to_string_lossy()))?
            .to_string_lossy();
        let mut img = image::open(&path)
            .chain_err(|| format!("Cannot open image `{}`", path.to_string_lossy()))?;
        let (width, height) = img.dimensions();

        let left = imageops::crop(&mut img, 0, 0, width / 2, height).to_image();
        let mut left_path = dir.to_path_buf();
        left_path.push(format!("{}-{}", idx + 1, filename));
        left.save(&left_path)
            .chain_err(|| format!("Cannot save left part of `{}`", &filename))?;

        let right = imageops::crop(&mut img, width / 2, 0, width, height).to_image();
        let mut right_path = dir.to_path_buf();
        right_path.push(format!("{}-{}", pages_count - idx, filename));
        right
            .save(&right_path)
            .chain_err(|| format!("Cannot save right part of `{}`", &filename))?;
    }

    Ok(())
}
