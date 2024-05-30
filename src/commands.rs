use crate::renamer::{rename_one, FORMS};
use std::fs;
use clap::Parser;
use chrono::Local;


#[derive(Parser, Debug)]
#[command(name = "Unicode Filename Normalizer")]
#[command(author = "ruanlj <ruanlj@live.com>")]
#[command(about = format!("Unicode normalize filenames in folder to form one of [{}]", FORMS.join(", ")))]
#[command(long_about = None)]
pub struct Args {
    /// Normalize form
    #[arg(short, long, value_parser = FORMS)]
    to_form: String,

    /// Path to be convert
    path: Vec<String>,

    /// Log file path
    #[arg(short, long, default_value = "convert.log")]
    log: String,

    /// Dry run convert
    #[arg(long)]
    dry_run: bool,
}

pub fn run_rename() {
    let args: Args = Args::parse();
    println!("Normalizing to {}, Paths: {:?}", args.to_form, args.path);
    let mut fd = fs::File::options().write(true).create(true).append(true).open(&args.log).unwrap();
    let today = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    for p in &args.path {
        rename_one(p, &mut fd, &args.to_form, args.dry_run, &today);
    }
    fd.sync_all().unwrap();
}
