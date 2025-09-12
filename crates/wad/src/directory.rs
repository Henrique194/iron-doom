use crate::sys;
use anyhow::Result;
use std::{path::Path, {env, path::PathBuf}};

pub static IWADS: &[&str; 2] = &["doom.wad", "doom1.wad"];

#[derive(Debug)]
pub struct IwadDirs {
    dirs: Vec<PathBuf>,
}

impl IwadDirs {
    pub fn try_new() -> Result<Self> {
        let mut iwad_dirs = Self { dirs: Vec::new() };

        // Look in the current directory. Doom always does this.
        iwad_dirs.add_dir(".");

        // Next check the directory where the executable is located.
        // This might be different from the current directory.
        let curr_dir = env::current_dir()?;
        iwad_dirs.add_dir(curr_dir);

        // Add DOOMWADDIR and directories from DOOMWADPATH if defined in environment.
        if let Ok(dir) = env::var("DOOMWADDIR") {
            iwad_dirs.add_dir(&dir);
        }
        if let Ok(path) = env::var("DOOMWADPATH") {
            iwad_dirs.add_path(&path, "");
        }

        // Add OS specific directories.
        sys::add_dirs(&mut iwad_dirs);

        Ok(iwad_dirs)
    }

    pub fn add_path_buf(&mut self, path: PathBuf) {
        self.dirs.push(path);
    }

    pub fn add_dir<T: Into<PathBuf>>(&mut self, dir: T) {
        self.add_path_buf(dir.into());
    }

    pub fn add_path(&mut self, path: &str, suffix: &str) {
        for mut path in env::split_paths(path) {
            path.push(suffix);
            self.add_path_buf(path);
        }
    }

    /// Name can either be the name of the IWAD or the absolute path to the IWAD.
    pub fn find_iwad(&self, name: &Path) -> Option<PathBuf> {
        if name.exists() {
            // Absolute path
            return Some(name.to_path_buf());
        }

        for dir in &self.dirs {
            if let Some(iwad_path) = Self::has_iwad(dir, name) {
                return Some(iwad_path);
            }
        }

        None
    }

    pub fn search_iwads(&self) -> Option<PathBuf> {
        for dir in &self.dirs {
            for iwad in IWADS {
                if let Some(iwad_path) = Self::has_iwad(dir, iwad) {
                    return Some(iwad_path);
                }
            }
        }
        None
    }

    fn has_iwad<P: AsRef<Path>>(dir: &Path, iwad_name: P) -> Option<PathBuf> {
        let iwad_name = iwad_name.as_ref();

        // As a special case, if this is in DOOMWADDIR or DOOMWADPATH,
        // the "directory" may actually refer directly to an IWAD file.
        if let (true, Some(base)) = (dir.exists(), dir.file_name()) {
            if base.eq_ignore_ascii_case(iwad_name) {
                return Some(dir.to_path_buf());
            }
        }

        // Construct a string for the full path
        let file_name = dir.join(iwad_name);
        if file_name.exists() {
            Some(file_name)
        } else {
            None
        }
    }
}
