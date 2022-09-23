use std::{fs::{File, self}, io::{Read, self, Write, Error, ErrorKind}, path::{PathBuf, Path}, ffi::OsStr};
use bstr::BString;
use chrono::{DateTime, Utc};

pub fn read_save(filename: &str) -> io::Result<BString> {
    let mut file: File = File::open(&filename.trim())?;
    let mut buffer: Vec<u8> = Vec::new();
    let eof_error: Error  = Error::from(ErrorKind::UnexpectedEof);
        
    // Read the whole file.
    file.read_to_end(&mut buffer)?;

    // Use a Byte String because FL saves are ANSI (Windows code page WinLatin1).
    let contents: BString = BString::from(buffer);

    if contents.is_empty() {
        Err(eof_error)
    } else {
        Ok(contents)
    }
}

pub fn write_out(save_dir: PathBuf, save_name: Option<&OsStr>, buf: String) -> io::Result<()> {
    let save_path: PathBuf = save_dir.join(save_name.unwrap());
    let mut fl_file: File = File::create(save_path)?;

    write!(fl_file, "{}", buf)?;
    Ok(())
}

pub fn backup_save(orig_path: &Path) {
    let now: DateTime<Utc> = Utc::now();
    let fl_date: String = format!("fl.{}.orig", now.format("%Y%m%d_%H%M%S"));
    let fl_backup: PathBuf = orig_path.with_extension(fl_date);

    println!();
    println!("Making backup of original save.");
    fs::copy(orig_path, &fl_backup).expect("Unable to create backup of save file.");
    println!("Backup complete: {}", fl_backup.display());
}