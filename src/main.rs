use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn print_usage() {
    println!("Usage: hexdump [-b -c -bc -h] [FILENAME]");
}

fn print_help(){
    println!("Usage: hexdump [-b -c -bc -h] [FILENAME]");
    println!("\t-b\tBinary mode");
    println!("\t-c\tColor mode");
    println!("\t-bc\tBinary mode and color mode");
    println!("\t-h\tHelp");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        print_usage();
        return;
    }
    if args[1] == "-h" {
        print_help();
        return;
    }

    let mut binary_mode = false;
    let mut color_mode = false;
    let filename = if args.len() == 3 {
        if args[1] != "-b" && args[1] != "-c" && args[1] != "-bc" {
            println!("Command not found: {}", args[1]);
            print_usage();
            return;
        }
        else if args[1] == "-b" {
            binary_mode = true;
        }else if args[1] == "-bc" {
            binary_mode = true;
            color_mode = true;
        } else {
            color_mode = true;
        }
        &args[2]
    } else if args.len() == 2 {
        &args[1]
    } else {
        print_usage();
        return;
    };

    if !std::path::Path::new(filename).exists() {
        println!("File not found: {}", filename);
        return;
    }

    let file = File::open(filename).expect("Failed to open file: {}", filename);
    let mut reader = BufReader::new(file);

    let mut buf = [0; 16];
    let mut address = 0;

    loop {
        let n = reader.read(&mut buf).expect("Failed to read from file {}", filename);
        if n == 0 {
            break;
        }

        if !binary_mode {
            print!("{:08x} ", address);
        }

        for i in 0..16 {
            if i < n {
                let val = buf[i];
                if binary_mode {
                    if color_mode {
                        if val == 0 {
                            print!("\x1B[38;5;244m{:08b}\x1B[0m ", val);
                        } else {
                            print!("{:08b} ", val);
                        }
                    } else {
                        print!("{:08b} ", val);
                    }
                } else if color_mode {
                    if val == 0 {
                        print!("\x1B[38;5;244m{:02x}\x1B[0m ", val);
                    } else {
                        print!("{:02x} ", val);
                    }
                } else {
                    print!("{:02x} ", val);
                }
            } else {
                print!("   ");
            }

            if i == 7 {
                print!(" ");
            }
        }

        if !binary_mode {
            print!(" ");

            for i in 0..n {
                let c = buf[i] as char;
                if c.is_ascii_alphanumeric() {
                    print!("{}", c);
                } else {
                    print!(".");
                }
            }
        }

        println!();
        address += 16;
    }
}