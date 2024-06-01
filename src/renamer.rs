use std::fs;
use std::io::Write;
use walkdir;
use clap::ValueEnum;

use unicode_normalization::UnicodeNormalization;


#[derive(Debug, Clone, ValueEnum)]
pub enum NormalForm {
    NFC,
    NFD,
    NFKC,
    NFKD,
}

impl NormalForm {
    fn matched(&self, s: &str) -> bool {
        match *self {
            Self::NFC => unicode_normalization::is_nfc(&s),
            Self::NFD => unicode_normalization::is_nfd(&s),
            Self::NFKC => unicode_normalization::is_nfkc(&s),
            Self::NFKD => unicode_normalization::is_nfkd(&s),
        }
    }

    fn convert(&self, s: &str) -> String {
        match *self {
            Self::NFC => s.nfc().collect::<String>(),
            Self::NFD => s.nfd().collect::<String>(),
            Self::NFKC => s.nfkc().collect::<String>(),
            Self::NFKD => s.nfkd().collect::<String>(),
        }
    }

    pub fn normalize(&self, s: String) -> String {
        match self.matched(&s) {
            true => s,
            false => self.convert(&s)
        }
    }
}


pub fn rename_one(path: &String, log_fd: &mut fs::File, form: &NormalForm, dry_run: bool, today: &String) {
    for entry in walkdir::WalkDir::new(path).contents_first(true) {
        let entry = match entry {
            Ok(i) => i,
            Err(i) => {println!("WARN\t{:?}", i); continue;}
        };
        let filename = match entry.file_name().to_str() {
            Some(i) => i.to_string(),
            _ => {println!("SKIP\t{}", entry.path().display()); continue;}
        }; entry.file_name();

        let src = entry.path();
        let new_filename = form.normalize(filename.clone());
        if filename == new_filename {
            continue;
        }
        let dst = entry.path().parent().unwrap().join(new_filename);
        let msg = format!("{} -> {}", src.display(), dst.display());
        if dry_run {
            println!("DRY_RUN\t{}", msg);
            continue;
        }
        match fs::rename(&src, &dst) {
            Ok(_) => {
                println!("SUCC\t{}", msg);
                log_fd.write_all(format!("[{}]\t", today).as_bytes()).unwrap();
                log_fd.write_all(msg.as_bytes()).unwrap();
                log_fd.write_all(b"\n").unwrap();
                log_fd.sync_data().unwrap();
            },
            Err(i) => println!("FAIL\t{}\t{}", src.display(), i),
        };
    }
}
