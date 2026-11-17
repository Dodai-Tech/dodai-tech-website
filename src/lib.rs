//! The Dodai Tech website generator.
//!
//! [`site::build`] writes every page into a directory; [`copy_dir`] lays the hand-made files of
//! `static/` under it first. `main` does both into `public/`, which is what gets deployed.
pub mod charts;
pub mod shaders;
pub mod site;

use std::{fs, io, path::Path};

/// Copies the directory `from` into `to` (created if needed), replacing files of the same name.
///
/// # Errors
/// Returns the first I/O error, such as an unreadable source directory.
pub fn copy_dir(from: &Path, to: &Path) -> io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}
