use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {

    let mut handles: Vec<JoinHandle<()>> = vec![];
    for i in 0..10 {
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
        });
        handles.push(handle)
    }

    let mut completed_threads = 0;
    for handle in handles {
        handle.join().unwrap();
        completed_threads += 1;
    }

    if completed_threads != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

}