use std::{borrow::Borrow, sync::{mpsc, Arc, Mutex}};
mod ui;

static mut Q : Option<mpsc::Sender<String>> = None;
fn main() {
    let q_set: (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    let sender : mpsc::Sender<String> = q_set.0;
    let receiver = q_set.1;
    unsafe { 
        // This is safe because we are sure that Q is not being accessed by any other thread
        Q = Some(sender.clone()); 
    };

    let user_thread = ui::user_input::user_input::user_thread(sender, receiver);
    
}
