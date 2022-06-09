use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write};
/// https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html
fn sha256_digest<R: Read>(mut reader: R) -> Digest {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    context.finish()
}
fn main() {
    
    let hash = {
        let start = std::time::Instant::now();
        let path = "./firecracker";

        let input = File::open(path).unwrap();
        let reader = BufReader::new(input);
        let digest = sha256_digest(reader);
        let out = HEXUPPER.encode(digest.as_ref());
        println!("SHA-256 digest is {}",out);
        println!("{:?}",start.elapsed());
        out
    };
    

    let new_hash = {
        let start = std::time::Instant::now();
        let path = "./firecracker copy";

        let input = File::open(path).unwrap();
        let reader = BufReader::new(input);
        let digest = sha256_digest(reader);
        let out = HEXUPPER.encode(digest.as_ref());
        println!("SHA-256 digest is {}",out);
        println!("{:?}",start.elapsed());
        out
    };
    {
        let start = std::time::Instant::now();
        assert_eq!(hash,new_hash);
        println!("{:?}",start.elapsed());
    }
    
}