use std::{fs::File, io::{Read, self, Write}, convert::TryInto};
use bstr::{BString, ByteVec, ByteSlice};

/*fn read_save(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename.trim())?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}*/

fn read_save(filename: &str) -> io::Result<BString> {
    let mut file = File::open(&filename.trim())?;
    let mut buffer = Vec::new();
    
    // Read the whole file
    file.read_to_end(&mut buffer)?;

    // Use a Byte String because FL saves are ANSI (Windows code page WinLatin1)
    let contents = BString::from(buffer);
    Ok(contents)
}

fn decrypt (buffer: BString) {
    let mut len =  4;
    let mut my_iter = 0;
    let gene = [0x0047, 0x0045, 0x004E, 0x0045];
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
        
            len = len + 1;
            my_iter = my_iter + 1;
        }
    
        let decipher_save = std::str::from_utf8(&decipher_buf);

        //println!("{:?}", my_buf);
        //println!("Start of Conversion.");
        //println!("{}", decipher_save.expect("msg"));

        write_out(decipher_save.unwrap_or("borked").to_string());
    }
}

/*fn decrypt(buffer: Chars<'_>) {
    let mut len =  4;
    let mut my_iter = 0;
    //let my_buf = &buffer.as_str();
    let my_buf: Vec<Chars> = Vec::new();
    my_buf.push(buffer);
    let my_buf_bytes: Vec<u8> = my_buf.as_bytes().to_vec();
    let byte_buf = my_buf.as_bytes().len() - 4;
    
    //let mut byte_buf = Vec::new();
    //byte_buf.push(my_buf_bytes.get(my_buf.as_bytes().len() - 4));

    let gene = [0x0047, 0x0045, 0x004E, 0x0045];
    //let gene = ["G".as_bytes(), "e".as_bytes(), "n".as_bytes(), "e".as_bytes()];

    //if my_buf.contains("FLS1") {
    if my_buf.contains("FLS1") {
        for i in my_buf.chars().into_iter() {
            print!("{}", i);
        }

        println!(" ");
        println!("{:?}", byte_buf);

        let mut decipher_buf: Vec<u8> = Vec::new();

        //decipher_buf.push( byte_buf.len() - 4);

        while len < byte_buf {
            let gene_cipher: u8 = ((gene[len % 4] + len) % 256).try_into().unwrap();
            
            decipher_buf.push(my_buf_bytes.get(len).unwrap() ^ (gene_cipher | 0x80));
            //decipher_buf.push(my_buf_byte[len]);// ^ (gene_cipher | 0x80).try_into());
            //decipher_buf.push((gene_cipher | 0x80).try_into().unwrap());
          
            len = len + 1;
            my_iter = my_iter + 1;
        }

        let decipher_save = std::str::from_utf8(&decipher_buf);

        println!("{:?}", byte_buf);
        println!("Start of Conversion.");
        println!("{}", decipher_save.expect("msg"));
    };

    /*for i in 0..len {
        let key: Vec<&str> = Vec::new();
        key.push("Gene");

        let buff_iter = buffer.iter();
        buff_iter ^= key[i & 3];
        //buffer[i] ^= key[i & 3];
    };*/
}*/

fn write_out(buf: String) -> io::Result<()> {
    let mut buffer = File::create("D:\\Users\\Devin\\source\\tmp\\my_save.fl")?;
    //let mut w = Vec::new();
    write!(buffer, "{}", buf)?;
    Ok(())
}

fn main() {
    // this method needs to be inside main() method
    //env::set_var("RUST_BACKTRACE", "1");
    let _debug = false;

    let mut fl_path = String::new();
    let mut _fl_save_decrypt: Vec<char> = Vec::new();

    println!("Input Save Path:");
    io::stdin().read_line(&mut fl_path).expect("Cannot read input.");

    println!("{:?}", &fl_path.trim());
    //println!("{:?}", read_save(&fl_path));

    let _fl_save_crypt = read_save(&fl_path);

    let fl_save = read_save(&fl_path);

    //let char_buffer = fl_save.chars();

    // Debug Output of save file buffer.
    /*if debug {
        for i in char_buffer {
            print!("{}", i);
        }
    }*/

    decrypt(fl_save.unwrap()); 

    //for char in fl_save_crypt.unwrap_or("Borked".to_string()).chars().next_back(){
    //        fl_save_decrypt.push(char);
    // }

    // let s: String = fl_save_decrypt.into_iter().collect();

    // println!("{}", s);
}