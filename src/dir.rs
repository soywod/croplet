use error_chain::error_chain;
use std::{
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

error_chain! {
    foreign_links {
        Io(io::Error);
    }
}

fn is_a_picture(file: DirEntry) -> Option<PathBuf> {
    let path = file.path();
    let ext = path.extension()?.to_str()?;

    if ["jpg", "jpeg", "tiff"].iter().any(|&e| e == ext) {
        Some(path)
    } else {
        None
    }
}

pub fn get_picture_paths<'a>(dir: &'a Path) -> Result<Vec<PathBuf>> {
    let mut imgs = fs::read_dir(dir)
        .chain_err(|| format!("Cannot read directory `{}`", dir.to_string_lossy()))?
        .filter_map(|f| f.ok().and_then(is_a_picture))
        .collect::<Vec<_>>();

    imgs.sort();

    Ok(imgs)
}
