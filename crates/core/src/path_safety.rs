//! Path validation for file-system operations.
//!
//! The CLI operates on files in the current project directory. These helpers
//! canonicalize paths and ensure they cannot escape that directory before they
//! are passed to file-system APIs.

use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::{Component, Path, PathBuf};

fn current_directory() -> Result<PathBuf> {
    std::env::current_dir()?.canonicalize()
}

fn path_text(path: &Path) -> Result<&str> {
    path.to_str().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("path '{}' is not valid UTF-8", path.display()),
        )
    })
}

fn reject_parent_references(path: &Path) -> Result<()> {
    let path_text = path_text(path)?;
    if path_text.contains("..") {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "path '{}' contains a parent-directory reference",
                path.display()
            ),
        ));
    }
    Ok(())
}

/// Resolve an existing relative file only when it is inside the current project directory.
pub fn existing_file(path: &Path) -> Result<PathBuf> {
    reject_parent_references(path)?;
    if path.is_absolute() {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            format!("path '{}' must be relative", path.display()),
        ));
    }

    let project_root = current_directory()?;
    let path = project_root.join(path).canonicalize()?;
    if !path.starts_with(&project_root) {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "path '{}' is outside the current project directory",
                path.display()
            ),
        ));
    }
    Ok(path)
}

/// Read a project-local file after validating its path at the file-system boundary.
pub fn read_project_file(path: &Path) -> Result<String> {
    let path_text = path_text(path)?;
    if path_text.contains("..") {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "path '{}' contains a parent-directory reference",
                path.display()
            ),
        ));
    }

    let project_root = current_directory()?;
    let path = project_root.join(path).canonicalize()?;
    if !path.starts_with(&project_root) {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "path '{}' is outside the current project directory",
                path.display()
            ),
        ));
    }
    fs::read_to_string(path)
}

/// Resolve an existing relative output directory inside the current project directory.
pub fn output_directory(path: &Path) -> Result<PathBuf> {
    let path_text = path_text(path)?;
    if path_text.contains("..") {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "path '{}' contains a parent-directory reference",
                path.display()
            ),
        ));
    }

    if path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "output directory '{}' must be relative and project-local",
                path.display()
            ),
        ));
    }

    let project_root = current_directory()?;
    let directory = project_root.join(path).canonicalize()?;
    if !directory.starts_with(&project_root) {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "path '{}' is outside the current project directory",
                directory.display()
            ),
        ));
    }
    Ok(directory)
}

/// Write an SVG only when its parent directory is inside the current project directory.
pub fn write_project_file(path: &Path, content: String) -> Result<()> {
    let path_text = path_text(path)?;
    if path_text.contains("..") {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "path '{}' contains a parent-directory reference",
                path.display()
            ),
        ));
    }

    let project_root = current_directory()?;
    let parent = path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .canonicalize()?;
    if !parent.starts_with(&project_root) {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "path '{}' is outside the current project directory",
                parent.display()
            ),
        ));
    }

    let file_name = path.file_name().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("output path '{}' does not name a file", path.display()),
        )
    })?;
    fs::write(parent.join(file_name), content)
}

/// Remove a project-local file after validating its path at the file-system boundary.
pub fn remove_project_file(path: &Path) -> Result<()> {
    let path_text = path_text(path)?;
    if path_text.contains("..") {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "path '{}' contains a parent-directory reference",
                path.display()
            ),
        ));
    }

    let project_root = current_directory()?;
    let path = path.canonicalize()?;
    if !path.starts_with(&project_root) {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "path '{}' is outside the current project directory",
                path.display()
            ),
        ));
    }
    fs::remove_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_parent_references() {
        assert!(reject_parent_references(Path::new("../outside.svg")).is_err());
    }
}
