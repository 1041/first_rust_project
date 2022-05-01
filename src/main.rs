use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut handlers = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        
    handlers.push(thread::spawn(move || {
            let mut data2 = data_ref.lock().unwrap();
            data2[x] += 1; 
        }));
    }

    for handler in handlers {
        let _ = handler.join();
    }

    dbg!(data);

     println!("Hello, world!");
}