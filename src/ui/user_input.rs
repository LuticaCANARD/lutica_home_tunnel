
pub(crate) mod user_input{
    use std::sync::{Arc, Mutex, mpsc};
    use std::thread::JoinHandle;
    use std::{io, thread};
    pub fn user_input(){
        println!("Enter your name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        println!("Hello, {}", name);
    }
    
    pub fn user_thread(sender: mpsc::Sender<String>, receiver: mpsc::Receiver<String>)->JoinHandle<()>
    {
        return thread::spawn(move || {
            loop {
                let msg = receiver.recv().unwrap();
                println!("Received: {}", msg);
            }
        });
    }
}
