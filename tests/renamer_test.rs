use std::fs;
use std::fs::File;
use chrono::Local;
use tempdir::TempDir;
use unicode_normalizer::renamer::*;

#[test]
fn test_normalize() {
    let nfc = String::from("\u{3076}");
    let nfd = String::from("\u{3075}\u{3099}");
    let nfkc = String::from("\u{00c5}");
    let nfkd = String::from("A\u{030a}");
    println!("{:#} {:#} {:#} {:#}", nfc, nfd, nfkc, nfkd);
    assert_eq!(NormalForm::NFD.normalize(nfc.clone()), nfd.clone());
    assert_eq!(NormalForm::NFC.normalize(nfd.clone()), nfc.clone());
    assert_eq!(NormalForm::NFKD.normalize(nfkc.clone()), nfkd.clone());
    assert_eq!(NormalForm::NFKC.normalize(nfkd.clone()), nfkc.clone());
}

#[test]
fn test_rename_one() {
    let tmp_dir = TempDir::new("unicode_normalizer").unwrap();
    let fs_root = tmp_dir.path().join("files");
    let log_file = tmp_dir.path().join("convert.log");
    fs::create_dir_all(fs_root.join("b")).unwrap();
    fs::create_dir_all(fs_root.join("a/aa")).unwrap();
    println!("test-dir: {:?}", fs_root);

    let nfc = String::from("1-\u{3076}.txt");
    let nfd = String::from("2-\u{3075}\u{3099}.txt");
    File::create(fs_root.join("a").join(&nfc)).unwrap();
    File::create(fs_root.join("b").join(&nfd)).unwrap();
    File::create(fs_root.join("a/aa").join(&nfc)).unwrap();
    File::create(fs_root.join("a/aa").join(&nfd)).unwrap();

    let mut log_fd = fs::File::options().write(true).create(true).append(true).open(&log_file).unwrap();
    let form = NormalForm::NFC;
    let today = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let path = fs_root.as_path().display().to_string();
    rename_one(&path, &mut log_fd, &form, false, &today);
    log_fd.sync_all().unwrap();

    let expected1 = format!("[{}]\t{} -> {}", today,
        fs_root.join("a/aa").join(&nfd).as_path().display(),
        fs_root.join("a/aa").join(&form.normalize(nfd.clone())).as_path().display(),
    );
    let expected2 = format!("[{}]\t{} -> {}", today,
        fs_root.join("b").join(&nfd).as_path().display(),
        fs_root.join("b").join(&form.normalize(nfd.clone())).as_path().display(),
    );
    let out_a = format!("{}\n{}\n", expected1, expected2);
    let out_b = fs::read_to_string(&log_file).unwrap();
    assert!(out_a.eq(&out_b));
    tmp_dir.close().unwrap();
}
