#![allow(dead_code)]
use chrono::Utc;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};


pub struct FileWriter {
    // outfile_name: PathBuf,
    outfile_writer: BufWriter<File>,
}

impl FileWriter {
    pub fn new(outfile_prefix: &str) -> Self {
        
        let tmstmp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let filename = if outfile_prefix.contains("xml") {
            format!("uitree_{}_{}.xml", outfile_prefix, tmstmp)
        } else {
            format!("uitree_{}_{}.txt", outfile_prefix, tmstmp)
        };

                
        let err_msg = format!("Unable to create file: {}", filename);

        let f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&filename)
            .expect(&err_msg);
        let outfile_writer = BufWriter::new(f);

        FileWriter { outfile_writer }
    }

    pub fn write(&mut self, content: &str) {
        self.outfile_writer.write_all(content.as_bytes())
            .expect("Unable to write to file");
    }
    
}


pub fn execute_with_timeout<T, F>(timeout_ms: u64, f: F) -> Option<T> 
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    
    // Spawn the closure in a separate thread
    let _handle = thread::spawn(move || {
        let result = f();
        let _ = tx.send(result);
    });

    // Wait for either the timeout or the result
    match rx.recv_timeout(Duration::from_millis(timeout_ms)) {
        Ok(result) => {
            // Result received within timeout
            Some(result)
        }
        Err(_) => {
            // Timeout occurred
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_execute_with_timeout() {
        // Example 1: Operation completes within timeout
        let result = execute_with_timeout(1000, || {
            thread::sleep(Duration::from_millis(500));
            42
        });
        assert_eq!(result, Some(42));

        // Example 2: Operation times out
        let result = execute_with_timeout(1000, || {
            thread::sleep(Duration::from_millis(2000));
            42
        });
        assert_eq!(result, None);
    }
}