use std::{fs::File, io::{Read, self, Write, stdin}, convert::TryInto, env::{current_dir}, path::{PathBuf}, str::FromStr};
use bstr::{BString, ByteSlice};
use regex::{Regex, Captures};

fn read_save(filename: &str) -> io::Result<BString> {
    let mut file: File = File::open(&filename.trim())?;
    let mut buffer: Vec<u8> = Vec::new();
        
    // Read the whole file.
    file.read_to_end(&mut buffer)?;

    // Use a Byte String because FL saves are ANSI (Windows code page WinLatin1).
    let contents = BString::from(buffer);

    Ok(contents)
}

fn decrypt (buffer: BString) -> io::Result<String> {
    // First 4 bytes of the file "FLS1" to skip.
    let mut len =  4;
    let mut my_iter = 0;

    // "Gene, Gene, The Cinnabon Machine"
    let gene = [0x0047, 0x0065, 0x006E, 0x0065];

    let my_buf = &buffer;
    let byte_buf_len = my_buf.len();
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
    let re: Regex = Regex::new(r"MissionNum.*(.+)").unwrap();

    let mission_cap: Captures = re.captures(&buf).expect("The 'MissionNum' parameter was not found.");
    let mission_orig_str: &str = mission_cap.get(0).unwrap().as_str();
    let mission_num_str: &str = mission_cap.get(1).unwrap().as_str();
    let mission_num: i32 = FromStr::from_str(mission_num_str).unwrap();
    let mission_inc: i32 = mission_num + 1;
    let new_mission: String = mission_orig_str.replace(mission_num_str, mission_inc.to_string().as_str());
    
    let delta: &str = "delta_worth = -1.000000";
    let new_delta: &str = "delta_worth = 1.000000";
    
    let my_buf: String = buf.replace(mission_orig_str, new_mission.as_str());
    let my_buf: String = my_buf.replace(delta, new_delta);

    my_buf
}

fn write_out(save_dir: PathBuf, buf: String) -> io::Result<()> {
    let save_path: PathBuf = save_dir.join("my_save.fl");
    let mut fl_file = File::create(save_path)?;

    write!(fl_file, "{}", buf)?;
    Ok(())
}

fn save_prompt(usr_ans: &mut String, pwd: PathBuf, decrypted_save: String, fix: bool) {
    loop {
        println!("");
        println!("Save new file in current directory? (Y/N):");
        stdin().read_line(usr_ans).expect("Cannot read input.");
    
        if usr_ans.to_lowercase().contains("y") || usr_ans.to_lowercase().contains("yes") {
            if fix {
                if let Err(e) = write_out(pwd, fix_save(decrypted_save)) { println!("{:?}", e) }
                break;
            } else {
                if let Err(e) = write_out(pwd, decrypted_save) { println!("{:?}", e) }
                break;
            }
        } else if usr_ans.to_lowercase().contains("n") || usr_ans.to_lowercase().contains("no") {
            break;
        } else {
            println!("Answer must be \"Y, Yes, N, or No\".");
        }
    }
}

/*
TODO:
    * Get current save file name for use in new out file.
    * Backup current file.
    * Allow for dynamic save location.
    * GUI
*/

fn main() {
    let pwd: PathBuf = current_dir().expect("Cannot access current directory.");
    let mut usr_ans: String = String::new();
    let mut fl_path: String = String::new();

    println!("Input Save Path:");
    stdin().read_line(&mut fl_path).expect("Cannot read input.");

    let fl_save: BString = read_save(&fl_path).expect("Cannot find save file.");
    let decrypted_save: String = decrypt(fl_save).unwrap();

    //fix_save(decrypted_save);

    println!("");
    println!("1. Convert Save");
    println!("2. Fix Save and/or Convert");
    println!("0. Exit");
    stdin().read_line(&mut usr_ans).expect("Cannot read input.");

    if usr_ans.contains("1") {
        save_prompt(&mut usr_ans, pwd, decrypted_save, false);
    } else if usr_ans.contains("2") {
        save_prompt(&mut usr_ans, pwd, decrypted_save, true);
    } else {
        println!("Invalid Choice: {}", usr_ans)
    }
}