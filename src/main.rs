use std::fs;

fn main() {
    println!("`ls`");
    // Read the contents of a directory, returns `io::Result<Vec<Path>>`
    match fs::read_dir(".") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        },
    }

    let mut file_buff: Vec<u8> = Vec::new();

    match fs::read("test/test.txt") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(buff) => {
            file_buff = buff;
        }
    };

    for i in 0..file_buff.len() {
        print!("{} ", byte_to_hex_string(file_buff[i]));
    }

    println!("");

    for i in 0..file_buff.len() {
        print!("{}  ", byte_to_char(file_buff[i]));
    }

    println!("");
}

fn byte_to_hex_string(byte: u8) -> String {
    let high_nibble: u8 = &byte >> 4;
    let low_nibble: u8 = &byte & 0x0f;

    let mut res: String = String::from(nybble_to_char(high_nibble));

    res.push(nybble_to_char(low_nibble));

    return res;
}

fn nybble_to_char(nybble: u8) -> char {
    if nybble < 16 {
        if nybble < 10 {
            return (nybble | 0x30) as char;
        } else {
            return (nybble + 55) as char;
        }
    } else {
        panic!("Nybble should be number less than 16!");
    }
}

fn byte_to_char(byte: u8) -> char {
    if byte >= 32 && byte < 128 {
        return byte as char;
    } else {
        return 26 as char;
    }
}