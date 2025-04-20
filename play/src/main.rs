use std::sync::{mpsc, Arc, Mutex};
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

    // ===== 4. More messages between channel.
    // let (sender, receiver) = mpsc::channel();

    // thread::spawn(move || {
    //     let messages = vec!["Hello ", "World! ", "I am from sub thread."];

    //     for message in messages {
    //         sender.send(message.to_string()).unwrap();
    //     }
    // });

    // let mut contents = String::new();

    // for received in receiver {
    //     contents.push_str(&received);
    // }

    // println!("Received from sub thread: {}", contents);

    // ===== 5. Shared Data Between threads

    // Arc = 原子引用计数（用于在线程间安全共享）
    // Mutex = 互斥锁（确保一次只有一个线程可以访问数据）
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        // clone arc for each thread
        let cloned_counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = cloned_counter.lock().unwrap();
            *num += 1;
            // 离开作用域时自动释放锁
        });
        // if we handle.join here, it will wait for it to complete.
        handles.push(handle);
    }

    println!("Waiting for all the threads to complete...");
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter result: {}", *counter.lock().unwrap());
}
