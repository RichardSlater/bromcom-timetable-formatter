//! Path validation for file-system operations.
//!
//! The CLI operates on files in the current project directory. These helpers
//! canonicalize paths and ensure they cannot escape that directory before they
//! are passed to file-system APIs.

use std::io::{Error, ErrorKind, Result};
use std::path::{Component, Path, PathBuf};

fn current_directory() -> Result<PathBuf> {
    std::env::current_dir()?.canonicalize()
}

fn reject_outside_project(path: &Path, project_root: &Path) -> Result<()> {
    if path.starts_with(project_root) {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "path '{}' is outside the current project directory",
                path.display()
            ),
        ))
    }
}

/// Resolve an existing file only when it is inside the current project directory.
pub fn existing_file(path: &Path) -> Result<PathBuf> {
    let project_root = current_directory()?;
    let path = path.canonicalize()?;
    reject_outside_project(&path, &project_root)?;
    Ok(path)
}

/// Create and resolve a relative output directory inside the current project directory.
pub fn output_directory(path: &Path) -> Result<PathBuf> {
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "output directory '{}' is outside the current project directory",
                path.display()
            ),
        ));
    }

    let directory = current_directory()?.join(path);
    std::fs::create_dir_all(&directory)?;
    let directory = directory.canonicalize()?;
    reject_outside_project(&directory, &current_directory()?)?;
    Ok(directory)
}

/// Resolve an output file only when its parent directory is inside the current project directory.
///
/// The parent directory must already exist. This prevents a path such as
/// `../../outside.svg` from being used to create or overwrite files outside the project.
pub fn output_file(path: &Path) -> Result<PathBuf> {
    let project_root = current_directory()?;
    let parent = path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .canonicalize()?;
    reject_outside_project(&parent, &project_root)?;

    let file_name = path.file_name().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("output path '{}' does not name a file", path.display()),
        )
    })?;

    Ok(parent.join(file_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_paths_outside_project() {
        let outside = if cfg!(windows) {
            Path::new("C:\\Windows")
        } else {
            Path::new("/tmp")
        };
        assert!(reject_outside_project(outside, Path::new("/project")).is_err());
    }
}
