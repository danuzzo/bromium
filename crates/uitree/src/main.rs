
mod macros;

use std::sync::mpsc::channel;
use std::thread;
use std::sync::mpsc::{Receiver, Sender};
use uitree::{UITree, get_all_elements};



fn main() {

    let (tx, rx): (Sender<_>, Receiver<UITree>) = channel();
    printfmt!("Spawning separate thread to get ui tree");
    thread::spawn(|| {
        get_all_elements(tx, None);
    });
    printfmt!("Spawned separate thread to get ui tree");
    
    let ui_tree: UITree = rx.recv().unwrap();
    printfmt!("done getting ui tree");
    printfmt!("No of elemetns in UI Tree: {:#}", ui_tree.get_elements().len());
    
    // ui_tree.for_each(|_index, element| {
    //     printfmt!("Element: {:#?}", element);
    // });
}
