use error_chain::error_chain;
use image::imageops;
use std::path::Path;

error_chain! {}

pub fn process<P>(
    dir: P,
    margin: u32,
    chunk_prefix: &str,
    idx: usize,
    path: P,
    pages_count: usize,
    scanned_pages_len: usize,
) -> Result<()>
where
    P: AsRef<Path>,
{
    let even_idx = idx % 2 == 0;
    let filename = path
        .as_ref()
        .file_name()
        .ok_or_else(|| format!("Invalid filename `{}`", path.as_ref().to_string_lossy()))?
        .to_string_lossy();

    let img = image::open(&path)
        .chain_err(|| format!("Cannot open image `{}`", path.as_ref().to_string_lossy()))?;

    let mut img = if even_idx {
        imageops::rotate90(&img)
    } else {
        imageops::rotate270(&img)
    };
    let (width, height) = img.dimensions();

    let left = imageops::crop(
        &mut img,
        margin,
        margin,
        width / 2 - margin,
        height - margin,
    )
    .to_image();
    let cursor = if even_idx { pages_count - idx } else { idx + 1 };
    let prefix = format!("{:0fill$}", cursor, fill = scanned_pages_len);
    let left_path = dir
        .as_ref()
        .join(format!("{}__{}_{}", chunk_prefix, prefix, filename));
    left.save(&left_path)
        .chain_err(|| format!("Cannot save left part of `{}`", &filename))?;

    let right = imageops::crop(
        &mut img,
        width / 2 - margin,
        margin,
        width - margin,
        height - margin,
    )
    .to_image();
    let cursor = if even_idx { idx + 1 } else { pages_count - idx };
    let prefix = format!("{:0fill$}", cursor, fill = scanned_pages_len);
    let right_path = dir
        .as_ref()
        .join(format!("{}__{}_{}", chunk_prefix, prefix, filename));
    right
        .save(&right_path)
        .chain_err(|| format!("Cannot save right part of `{}`", &filename))?;

    Ok(())
}
