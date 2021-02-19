use std::fs::File;
use std::io::Read;
use crate::elf::ELFHeader;
use std::borrow::Borrow;

mod elf;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: elfreader <path-to-elf-file>");
        return;
    }
    let filename = args.into_iter().take(2).last().expect("args should contain 2 entries");
    println!("parsing ELF header of file {}", filename);
    let mut file = match File::open(filename.as_str()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("can not open file {}", filename);
            eprintln!("reason: {}", e);
            return;
        }
    };
    let mut buf = [0; 512];
    let bytes_read = file.read(&mut buf);
    match bytes_read {
        Err(e) => {
            eprintln!("error reading file. Abort");
            eprintln!("reason: {}", e);
            return;
        }
        _ => {}
    };
    let bytes = &buf;
    let header = ELFHeader::from_bytes(bytes);
    let header =  match header {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Error parsing ELF header. Reason:");
            eprintln!("{:?}", e);
            return;
        }
    };
    println!("Header parsed successfully");
    println!("the content is: {:?}", header);
}