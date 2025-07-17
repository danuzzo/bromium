
mod macros;

use chrono::Utc;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
// use std::path::PathBuf;


use std::sync::mpsc::channel;
use std::thread;
use std::sync::mpsc::{Receiver, Sender};
use uitree::{UITree, get_all_elements};
use uitree::{UITreeIter, get_all_elements_iterative};

struct FileWriter {
    // outfile_name: PathBuf,
    outfile_writer: BufWriter<File>,
}

impl FileWriter {
    fn new(outfile_prefix: &str) -> Self {
        
        let tmstmp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let filename = format!("uitree_{}_{}.txt", outfile_prefix, tmstmp);
        // let mut outfile_name = PathBuf::new();
                
        let err_msg = format!("Unable to create file: {}", filename);

        let f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&filename)
            .expect(&err_msg);
        let outfile_writer = BufWriter::new(f);

        FileWriter { outfile_writer }
    }

    fn write(&mut self, content: &str) {
        self.outfile_writer.write_all(content.as_bytes())
            .expect("Unable to write to file");
    }
    
}

fn main() {

    // create file writers
    let mut file_writer_recursive = FileWriter::new("recursive_uitree");
    let mut file_writer_iterative = FileWriter::new("iterative_uitree");

    // recursive
    
    let (tx, rx): (Sender<_>, Receiver<UITree>) = channel();
    printfmt!("Spawning separate thread to get ui tree");
    thread::spawn(|| {
        get_all_elements(tx, None);
    });
    printfmt!("Spawned separate thread to get ui tree");
    
    let ui_tree: UITree = rx.recv().unwrap();
    printfmt!("done getting ui tree");
    printfmt!("No of elemetns in UI Tree: {:#}", ui_tree.get_elements().len());
    
    ui_tree.for_each(|_index, element| {
        // printfmt!("Element: {:#?}", element);
        // write to file
        file_writer_recursive.write(&format!("{:#?}\n", element));
    });


    // iterative
    let (tx_iter, rx_iter): (Sender<_>, Receiver<UITreeIter>) = channel();
    printfmt!("Spawning separate thread to get ui tree iteratively");
    thread::spawn(move || {
        get_all_elements_iterative(tx_iter, None);
    });
    printfmt!("Spawned separate thread to get ui tree iteratively");
    
    let ui_tree_iter: UITreeIter = rx_iter.recv().unwrap();
    printfmt!("done getting ui tree iteratively");
    printfmt!("No of elemetns in UI Tree Iter: {:#}", ui_tree_iter.get_elements().len());
    
    ui_tree_iter.for_each(|_index, element| {
        // printfmt!("Element: {:#?}", element);
        // write to file
        file_writer_iterative.write(&format!("{:#?}\n", element));
    });
    
}

