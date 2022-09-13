use std::{fs::{File, self}, io::{Read, self, Write, stdin}, convert::TryInto, env::current_dir, path::{PathBuf}, str::FromStr, process::{exit}, ffi::OsStr};
use bstr::{BString, ByteSlice};
use chrono::{DateTime, Utc};
use regex::{Regex, Captures};

fn read_save(filename: &str) -> io::Result<BString> {
    let mut file: File = File::open(&filename.trim())?;
    let mut buffer: Vec<u8> = Vec::new();
        
    // Read the whole file.
    file.read_to_end(&mut buffer)?;

    // Use a Byte String because FL saves are ANSI (Windows code page WinLatin1).
    let contents: BString = BString::from(buffer);

    Ok(contents)
}

fn decrypt (buffer: BString) -> io::Result<String> {
    // First 4 bytes of the file "FLS1" to skip.
    let mut len: usize =  4;
    let mut my_iter: usize = 0;

    // "Gene, Gene, The Cinnabon Machine."
    let gene: [usize; 4] = [0x0047, 0x0065, 0x006E, 0x0065];

    let my_buf: &BString = &buffer;
    let byte_buf_len: usize = my_buf.len();
    let mut decipher_buf: Vec<u8> = Vec::new();

    // Check for encrypted save.fl file header.
    if my_buf.contains_str("FLS1") {
        while len < byte_buf_len {
            let gene_cipher: u8 = ((gene[my_iter % 4] + my_iter) % 256).try_into().unwrap();
            
            decipher_buf.push(my_buf.get(len).unwrap() ^ (gene_cipher | 0x80));
        
            len += 1;
            my_iter += 1;
        }
    
        let decipher_save = std::str::from_utf8(&decipher_buf);

        // Return the deciphered save data.
        Ok(decipher_save.unwrap().to_string())

    } else { // Not encrypted, return original buffer data.
        Ok(my_buf.to_string())
    }
}

fn fix_save(buf: String) -> String {
    // Match 'MissionNum' line, group assigned value.
    let re: Regex = Regex::new(r"MissionNum.*(.+)").unwrap();
    // Capture the 'MissionNum' line from the save.
    let mission_cap: Captures = re.captures(&buf).expect("The 'MissionNum' parameter was not found.");
    // Save the entire line as a str.
    let mission_orig_str: &str = mission_cap.get(0).unwrap().as_str();
    // Save just the current value as a str.
    let mission_num_str: &str = mission_cap.get(1).unwrap().as_str();
    // Convert the value from str to i32.
    let mission_num: i32 = FromStr::from_str(mission_num_str).unwrap();
    // Increment the value by 1 to allow player to advance to the next mission.
    let mission_inc: i32 = mission_num + 1;
    // Replace the mission value in the original str with our new incremented value.
    let new_mission: String = mission_orig_str.replace(mission_num_str, mission_inc.to_string().as_str());
    
    let delta: &str = "delta_worth = -1.000000";
    let new_delta: &str = "delta_worth = 1.000000";
    
    let my_buf: String = buf.replace(mission_orig_str, new_mission.as_str());
    let my_buf: String = my_buf.replace(delta, new_delta);

    my_buf
}

fn write_out(save_dir: PathBuf, save_name: Option<&OsStr>, buf: String) -> io::Result<()> {
    let save_path: PathBuf = save_dir.join(save_name.unwrap());
    let mut fl_file = File::create(save_path)?;

    write!(fl_file, "{}", buf)?;
    Ok(())
}

fn backup_save(orig_path: &PathBuf, fl_name: Option<&OsStr>) {
    let now: DateTime<Utc> = Utc::now();
    let my_path: PathBuf = PathBuf::from(fl_name.unwrap());
    let fl_date: String = format!("fl.{}.orig", now.format("%Y%m%d_%H%M%S"));
    let fl_backup: PathBuf = my_path.with_extension(fl_date);

    println!();
    println!("Making backup of original save.");
    fs::copy(orig_path, &fl_backup).expect("Unable to create backup of save file.");
    println!("Backup complete: {}", fl_backup.display());
}

fn fl_options(fl_path: String, pwd: PathBuf, fl_save: BString, fix: bool) {
    let mut usr_ans: String = String::new();
    // Get the original save's path.
    let orig_path: PathBuf = PathBuf::from(fl_path.trim());
    // Get just the name for the original save.
    let fl_name: Option<&OsStr> = orig_path.file_name();
    let decrypted_save: String = decrypt(fl_save).expect("Unable to read save contents to buffer.");
        
    backup_save(&orig_path, fl_name);
             
    loop {
        println!();
        println!("Save new file in current directory? (Y/N):");
        stdin().read_line(&mut usr_ans).expect("Cannot read input.");
    
        let usr_ans: String = usr_ans.to_lowercase();

        if usr_ans.contains('y') || usr_ans.contains("yes") {
            if fix {
                if let Err(e) = write_out(pwd, fl_name, fix_save(decrypted_save)) { println!("{:?}", e) }
                break;
            } else {
                if let Err(e) = write_out(pwd, fl_name, decrypted_save) { println!("{:?}", e) }
                break;
            }
        } else if usr_ans.contains('n') || usr_ans.contains("no") {
            break;
        } else {
            println!("Answer must be \"Y, Yes, N, or No\".");
        }
    }
}

/*
    TODO:
        * Allow for dynamic save location. (50)
        * GUI (0)
*/

fn main() {
    let pwd: PathBuf = current_dir().expect("Cannot access current directory.");
    let mut usr_ans: String = String::new();
    let mut fl_path: String = String::new();

    println!("Input Save Path:");
    stdin().read_line(&mut fl_path).expect("Cannot read input.");

    let fl_save: BString = read_save(&fl_path).expect("Cannot find save file.");

    println!();
    println!("1. Convert Save");
    println!("2. Fix Save and/or Convert");
    println!("0. Exit");
    stdin().read_line(&mut usr_ans).expect("Cannot read input.");

    if usr_ans.contains('1') {
        fl_options(fl_path, pwd, fl_save, false);
    } else if usr_ans.contains('2') {
        fl_options(fl_path, pwd, fl_save, true);
    } else if usr_ans.contains('0') {
        exit(0);
    } else {
        println!("Invalid Choice: {}", usr_ans);
    }
}