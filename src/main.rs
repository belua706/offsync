use std::path::Path;

use clap::Parser;

#[derive(clap::Parser, Debug)]
struct CliArgs {
    /// The source directory to backup
    #[arg(short, long)]
    src_dir: String,

    /// The target directory where to place the backup
    #[arg(short, long)]
    out_dir: String,
}

#[derive(Debug)]
struct SyncError;

fn sync(src: &Path, target: &Path) -> Result<(), SyncError> {
    Ok(())
}

fn main() {
    println!("Hello, world!");

    let cli_args = CliArgs::parse();

    println!(
        "Backing up the directory {} into: {}",
        cli_args.src_dir, cli_args.out_dir
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        collections::HashMap,
        ops::ControlFlow,
        os::unix::fs::MetadataExt,
        path::{Path, PathBuf},
        str::FromStr,
    };
    use tempfile::NamedTempFile;
    use tempfile::TempDir;
    use walkdir::WalkDir;

    const TMP_DIR_PARENT_DIR: &str = "/tmp/offsync";

    #[derive(Debug)]
    enum DiffError {
        SourceIo {
            path: PathBuf,
            error: std::io::Error,
        },
    }

    #[derive(Debug)]
    enum SyncMismatch {
        /// A file not found in the remote.
        MissingInTarget { path: PathBuf },
        /// A file not found in the remote.
        ExtraInTarget { path: PathBuf },
        /// The file size doesn't match.
        Size {
            path: PathBuf,
            source: u64,
            target: u64,
        },
        /// The content of the file doesn't match.
        Content { file_name: String, diff: String },
        TargetIoError {
            path: PathBuf,
            error: std::io::Error,
        },
    }

    fn create_temp_dir() -> TempDir {
        let parent_dir = PathBuf::from_str(TMP_DIR_PARENT_DIR).unwrap();
        std::fs::create_dir_all(&parent_dir).unwrap();
        let new_dir = TempDir::new_in(&parent_dir).unwrap();

        NamedTempFile::new_in(&new_dir).unwrap();
        NamedTempFile::new_in(&new_dir).unwrap();

        new_dir
    }

    fn index_dir<F, E>(root: &Path, mut on_error: F) -> Result<HashMap<PathBuf, u64>, E>
    where
        F: FnMut(PathBuf, std::io::Error) -> ControlFlow<E>,
    {
        let mut map = HashMap::new();

        for entry in WalkDir::new(root) {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => match on_error(root.to_path_buf(), e.into()) {
                    ControlFlow::Continue(()) => continue,
                    ControlFlow::Break(err) => return Err(err),
                },
            };

            if entry.file_type().is_file() {
                let rel = match entry.path().strip_prefix(root) {
                    Ok(p) => p.to_path_buf(),
                    Err(_) => continue,
                };

                match std::fs::metadata(entry.path()) {
                    Ok(meta) => {
                        map.insert(rel, meta.len());
                    }
                    Err(e) => match on_error(rel, e) {
                        ControlFlow::Continue(()) => {}
                        ControlFlow::Break(err) => return Err(err),
                    },
                }
            }
        }

        Ok(map)
    }

    // Source IO error is fatal,
    // target IO error counts as a SyncMismatch.
    fn diff_dirs(source: &Path, target: &Path) -> Result<Vec<SyncMismatch>, DiffError> {
        let mut mismatches = Vec::new();

        let source_index = index_dir(source, |path, error| {
            ControlFlow::Break(DiffError::SourceIo { path, error })
        })?;

        let target_index = index_dir(target, |path, error| {
            mismatches.push(SyncMismatch::TargetIoError { path, error });
            ControlFlow::Continue(())
        })?;

        for (path, src_size) in &source_index {
            match target_index.get(path) {
                None => mismatches.push(SyncMismatch::MissingInTarget { path: path.clone() }),
                Some(tgt_size) if tgt_size != src_size => mismatches.push(SyncMismatch::Size {
                    path: path.clone(),
                    source: *src_size,
                    target: *tgt_size,
                }),
                _ => {}
            }
        }

        for path in target_index.keys() {
            if !source_index.contains_key(path) {
                mismatches.push(SyncMismatch::ExtraInTarget { path: path.clone() });
            }
        }

        Ok(mismatches)
    }

    #[test]
    fn basic() {
        let src_dir = create_temp_dir();
        let target_dir = create_temp_dir();

        sync(src_dir.path(), target_dir.path()).unwrap();

        diff_dirs(src_dir.path(), target_dir.path()).unwrap();
    }
}
