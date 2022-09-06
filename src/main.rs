use std::fs;
use std::io;

fn main() {
    let mut file_buff: Vec<u8> = Vec::new();
    let file_path: String;

    println!("Enter the path to the file: ");
    file_path = read_input("this should be a string");

    match fs::read(file_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(buff) => {
            file_buff = buff;
        }
    };

    print_file_buffer(file_buff);
    

    println!("");
}

fn print_file_buffer(buff: Vec<u8>) {
    let mut index: usize;
    let mut answer: String;

    for j in 0..(buff.len() / 16 + 1) {
        if (j % 16) == 0 {
            println!("Block #{}", j / 16 + 1);
        }

        print!("{}\t|  ", j + 1);
        for i in 0..16 {
            index = j * 16 + i;
            if index < buff.len() {
                print!("{} ", byte_to_hex_string(buff[index]));
            } else {
                print!("   ");
            }
            if (i + 1) % 4 == 0 {
                print!(" ")
            }
            
            if i == 7 {
                print!(" ")
            }        
        }
        print!("|  ");
        for i in 0..16 {
            index = j * 16 + i;
            if index < buff.len() {
                print!("{}", byte_to_char(buff[index]));
            } else {
                print!(" ");
            }
        }
        println!("  |");
        
        if (j + 1) % 16 == 0 {
            println!("Enter to continue, any key to abort:");
            answer = read_input("qwer");
            if answer != "" {
                println!("answer: {}", answer);
                break;
            }
        }
    }
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
    } else if byte == 0 {
        return '░';
    } else if byte == 255 {
        return '█';
    } else {
        return 26 as char;
    }
}

fn read_input<T: std::str::FromStr>(error_message: &str) -> T {
    // generic function that reads input and checks for type (number or text)
    // the error handling should be here
    let mut input: String = String::new();
    let result: T;
    let error: &str;

    if error_message.trim() == "" {
        error = "use correct value type, e.g. number";
    } else {
        error = error_message;
    }

    loop {
        io::stdin()
        .read_line(&mut input)
        .expect("failed to read the line");

        input = input.trim().to_string();

        match input.parse::<T>() {
            Ok(parsed_value) => {
                result = parsed_value;
                break;
            },
            Err(_) => {
                println!("{}", error);
                input = String::new();
                continue;
            },
        };
    }

    return result;
}