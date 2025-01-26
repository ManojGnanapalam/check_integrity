use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{self,Read, BufReader};

fn main(){
    let file_path = r"C:\Users\hasshing.txt";
    println!("{:?}",get_hash(file_path));
}

fn get_hash(file_path: &str) -> io::Result<String>{
    let file = File::open(file_path)?;
    
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();

    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    hasher.update(&buffer);

    Ok(format!("{:x}", hasher.finalize()))
}