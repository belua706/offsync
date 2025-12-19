mod fs;

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

fn main() {
    println!("Hello, world!");

    let cli_args = CliArgs::parse();

    println!(
        "Backing up the directory {} into: {}",
        cli_args.src_dir, cli_args.out_dir
    );
}
