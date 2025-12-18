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
        io::Write,
        path::{Path, PathBuf},
        str::FromStr,
    };
    use tempfile::NamedTempFile;
    use tempfile::TempDir;

    const TMP_DIR_PARENT_DIR: &str = "/tmp/offsync";

    #[derive(Debug)]
    struct DirDiff;

    fn create_temp_dir() -> TempDir {
        let parent_dir = PathBuf::from_str(TMP_DIR_PARENT_DIR).unwrap();
        std::fs::create_dir_all(&parent_dir).unwrap();
        let new_dir = TempDir::new_in(&parent_dir).unwrap();

        NamedTempFile::new_in(&new_dir).unwrap();
        NamedTempFile::new_in(&new_dir).unwrap();

        new_dir
    }

    fn compare_directories(src: &Path, target: &Path) -> Result<(), DirDiff> {
        Err(DirDiff {})
    }

    #[test]
    fn basic() {
        let src_dir = create_temp_dir();
        let target_dir = create_temp_dir();

        sync(src_dir.path(), target_dir.path()).unwrap();

        compare_directories(src_dir.path(), target_dir.path()).unwrap();
    }
}
