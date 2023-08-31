use crate::CHUNK_SIZE;
use crossbeam::channel::Sender;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

pub fn read_loop(infile: &str, stx: Sender<usize>, wtx: Sender<Vec<u8>>) -> Result<()> {
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        let _ = stx.send(num_read);
        if wtx.send(Vec::from(&buffer[..num_read])).is_err() {
            // false means stop the program gracefully
            break;
        }
    }
    let _ = stx.send(0);
    let _ = wtx.send(Vec::new());

    Ok(())
}
