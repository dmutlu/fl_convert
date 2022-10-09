use bstr::BString;
use chrono::{DateTime, Utc};
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{self, Error, ErrorKind, Read, Write},
    path::{Path, PathBuf},
};

pub fn read_save(filename: &str) -> io::Result<BString> {
    let mut file: File = File::open(&filename.trim())?;
    let mut buffer: Vec<u8> = Vec::new();
    let eof_error: Error = Error::from(ErrorKind::UnexpectedEof);

    // Read the whole file.
    file.read_to_end(&mut buffer)?;

    // Use a Byte String because FL saves are ANSI (Windows code page WinLatin1).
    let contents: BString = BString::from(buffer);

    if contents.is_empty() {
        Err(eof_error)
    } else {
        Ok(contents)
    }
} // End of read_save.

pub fn write_out(save_dir: PathBuf, save_name: Option<&OsStr>, buf: String) -> io::Result<()> {
    let save_path: PathBuf = save_dir.join(save_name.unwrap());
    let mut fl_file: File = File::create(save_path)?;

    write!(fl_file, "{}", buf)?;
    Ok(())
} // End of write_out.

pub fn backup_save(orig_path: &Path) -> Result<&'static str, &'static str> {
    let now: DateTime<Utc> = Utc::now();
    let fl_date: String = format!("fl.{}.orig", now.format("%Y%m%d_%H%M%S"));
    let fl_backup: PathBuf = orig_path.with_extension(fl_date);

    if let Ok(..) = fs::copy(orig_path, &fl_backup) {
        Ok("[INFO]: Backup complete.\r\n")
    } else {
        Err("[ERROR]: Unable to create backup of save file.\r\n")
    }
} // End of backup_save.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_rtn_contents() {
        let save_file: &str = "./src/res/test/good_save.fl";

        assert_eq!(bstr::BString::from("FLS1"), read_save(save_file).unwrap());
    }

    #[test]
    fn read_rtn_error() {
        let save_file: &str = "./src/res/test/empty_save.fl";

        assert!(read_save(save_file).is_err());
    }

    #[test]
    fn write_ok() {
        let save_dir: PathBuf = std::path::PathBuf::from("./src/res/test/out/write");
        let test_path: &Path = Path::new("./src/res/test/out/write");
        let file_name: &OsStr = OsStr::new("write_test.fl");
        let save_name: Option<&OsStr> = Some(file_name);
        let buf: String = "FLS1".to_string();

        fs::create_dir(test_path).expect("Could not make test output dir.");

        assert!(write_out(save_dir, save_name, buf).is_ok());

        fs::remove_dir_all(test_path).expect("Could not remove test output dir.");
    }

    #[test]
    fn write_rtn_error() {
        let save_dir: PathBuf = std::path::PathBuf::from("./src/res/test/out/null/");
        let file_name: &OsStr = OsStr::new("write_test.fl");
        let save_name: Option<&OsStr> = Some(file_name);
        let buf: String = "FLS1".to_string();

        assert!(write_out(save_dir, save_name, buf).is_err());
    }

    #[test]
    fn backup_ok() {
        let save_dir: PathBuf = std::path::PathBuf::from("./src/res/test/out/bkup");
        let test_path: &Path = Path::new("./src/res/test/out/bkup");
        let file_name: &OsStr = OsStr::new("bkup_test.fl");
        let save_name: Option<&OsStr> = Some(file_name);
        let buf: String = "FLS1".to_string();
        let orig_path: &Path = Path::new("./src/res/test/out/bkup/bkup_test.fl");

        fs::create_dir(test_path).expect("Could not make test output dir.");

        write_out(save_dir, save_name, buf).expect("Could not write to test output dir.");

        assert!(backup_save(orig_path).is_ok());

        fs::remove_dir_all(test_path).expect("Could not remove test output dir.");
    }

    #[test]
    fn backup_rtn_error() {
        let orig_path: &Path = Path::new("./src/res/test/out/bkup_fail_test.fl");

        assert!(backup_save(orig_path).is_err());
    }
}
