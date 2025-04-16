use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // ===== 1. Create a thread. =====
    // let handle = thread::spawn(|| {
    //     println!("I am a new thread.");

    //     thread::sleep(Duration::from_secs(2));

    //     println!("Sub thread done.");
    // });
    // handle.join().unwrap();
    // println!("Main thread done.");

    // ===== 2. Values between thread =====
    // let message = String::from("Hello!");
    // println!("Greet in main thread: {}", message);

    // let handle = thread::spawn(move || {
    //     println!("Greet from main thread: {}", message);
    // });

    // // This will panic!
    // // println!("var message moved to sub thread: {}", message);

    // handle.join().unwrap();

    // ===== 3. Channel between threads =====
    // let (sender, reveiver) = mpsc::channel();

    // thread::spawn(move || {
    //     let message = String::from("Hello!");

    //     // `message` moved to main thread.
    //     sender.send(message).unwrap();
    // });

    // let reveiced = reveiver.recv().unwrap();
    // println!("Greet from sub thread: {}", reveiced);

    // ===== More messages between channel.
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["Hello ", "World! ", "I am from sub thread."];

        for message in messages {
            sender.send(message.to_string()).unwrap();
        }
    });

    let mut contents = String::new();

    for received in receiver {
        contents.push_str(&received);
    }

    println!("Received from sub thread: {}", contents);
}
