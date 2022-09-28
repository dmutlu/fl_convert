use std::{io::{self, ErrorKind}, convert::TryInto, str::FromStr};
use bstr::{BString, ByteSlice};
use regex::{Regex};

pub fn decrypt(buffer: &BString) -> io::Result<String> {
    // First 4 bytes of the file "FLS1" to skip.
    let mut len: usize =  4;
    let mut my_iter: usize = 0;

    // "Gene, Gene, The Cinnabon Machine."
    let gene: [usize; 4] = [0x0047, 0x0065, 0x006E, 0x0065];

    let my_buf: &BString = buffer;
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
} // End of decrypt.

pub fn fix_save(buf: String) -> Result<String, ErrorKind> {
    // Match 'MissionNum' line, group assigned value.
    let re: Regex = Regex::new(r"MissionNum.*(.+)").unwrap();
    // Capture the 'MissionNum' line from the save.
    let result = re.captures(&buf);
    
    if let Some(..) = result {
        let mission_cap = result.unwrap();
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

        Ok(my_buf)
    } else {
        // 'MissionNum' is not found, throw error.
        Err(ErrorKind::NotFound)
    }
} // End of fix_save.