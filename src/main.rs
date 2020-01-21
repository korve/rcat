use std::fs::File;
use std::io::{Read, Write, stdout};
use std::io;
use std::env;

fn main() -> std::io::Result<()>{
    let args = env::args();
    let path = args.skip(1).next();
    match path {
        Some(path) => {
            let file = File::open(path)?;
            let mut buf_reader = io::BufReader::new(file);
            let mut buf = [0u8; 10];
            while buf_reader.read(&mut buf)? > 0 {
                stdout().lock().write(&mut buf)?;
                buf = [0u8; 10];
            }

            stdout().flush()?;
            Ok(())
        },
        None => {
            println!("please provide a path");
            Ok(())
        }
    }
}
