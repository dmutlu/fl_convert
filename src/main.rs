use std::{fs::File, io::{Read, self, Write}, convert::TryInto, process::exit};
use bstr::{BString, ByteSlice};

fn read_save(filename: &str) -> io::Result<BString> {
    let mut file: File = File::open(&filename.trim())?;
    let mut buffer: Vec<u8> = Vec::new();
    let mut _buffer_debug: Vec<u8> = Vec::new();
        
    //file.read_to_end(&mut buffer_debug)?;
    // Read the whole file
    file.read_to_end(&mut buffer)?;

    // Use a Byte String because FL saves are ANSI (Windows code page WinLatin1)
    let contents = BString::from(buffer);

    //println!("{:?}", buffer_debug);

    Ok(contents)
}

fn decrypt (buffer: BString) -> io::Result<String> {
    let mut len =  4;
    let mut my_iter = 0;
    let gene = [0x0047, 0x0065, 0x006E, 0x0065];
    let my_buf = &buffer;
    let byte_buf_len = my_buf.len();
    //let byte_buf = my_buf.as_bytes();

    //for i in my_buf.into_iter() {
    //    print!("{}", i);
    //}

    //println!("Buffer length: {:?}", byte_buf_len);

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

        //println!("{:?}", my_buf);
        //println!("Start of Conversion.");
        //println!("{}", decipher_save.expect("msg"));

        //write_out(decipher_save.unwrap_or("borked").to_string());

        // Return the deciphered save data.
        Ok(decipher_save.unwrap().to_string())
    } else {
        Err(exit(0))
    }
}

fn write_out(buf: String) -> io::Result<()> {
    let mut buffer = File::create("D:\\Users\\Devin\\source\\tmp\\my_save.fl")?;
    //let mut w = Vec::new();
    write!(buffer, "{}", buf)?;
    Ok(())
}

fn main() {
    let mut fl_path = String::new();
    let mut _fl_save_decrypt: Vec<char> = Vec::new();

    println!("Input Save Path:");
    io::stdin().read_line(&mut fl_path).expect("Cannot read input.");

    let _fl_save_crypt = read_save(&fl_path);

    let fl_save = read_save(&fl_path);

    write_out(decrypt(fl_save.unwrap()).unwrap());
}