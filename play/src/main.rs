use std::fs::File;
use std::io::{self, Read, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
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

    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..5 {
    //     // clone arc for each thread
    //     let cloned_counter = Arc::clone(&counter);

    //     let handle = thread::spawn(move || {
    //         let mut num = cloned_counter.lock().unwrap();
    //         *num += 1;
    //         // 离开作用域时自动释放锁
    //     });
    //     // if we handle.join here, it will wait for it to complete.
    //     handles.push(handle);
    // }

    // println!("Waiting for all the threads to complete...");
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Final counter result: {}", *counter.lock().unwrap());

    // ===== 6. concurrent files concat
    let files = vec!["files/file1.txt", "files/file2.txt", "files/file3.txt"];

    let (sender, receiver) = mpsc::channel();

    for file in files {
        let sender_cloned = sender.clone();
        let file_path = file.to_string();

        thread::spawn(move || {
            if let Ok(content) = read_file(&file_path) {
                println!("{} read, size: {}.", file_path, content.len());
                sender_cloned.send((content, file_path)).unwrap();
            } else {
                println!("Read file error: {}", file_path);
            }
        });
    }

    // 不 drop 的话，原始的 `sender` 仍然存在于主线程中，`receiver` 会一直等待可能的新消息
    // 通过 `drop(sender)` 显式释放原始发送者，你告诉通道系统："主线程不会再发送任何消息"。
    // 显式丢弃原始sender，这样当所有clone的sender完成工作后，
    // receiver才能知道不会再有新消息，从而正确结束接收循环
    drop(sender);

    let mut final_contents = String::new();

    for received in receiver {
        let (content, file_path) = received;
        println!("file {} received.", file_path);
        final_contents.push_str(&format!("===== {} =====\n", file_path));
        final_contents.push_str(&content);
        final_contents.push_str("===== file end =====\n");
        final_contents.push_str("\n");
    }
    println!("All files read:\n{}", final_contents);

    let mut output_file = File::create("files/concat.txt")?;
    output_file.write_all(final_contents.as_bytes())?;
    println!("All files concat and write to concat.tx.");

    Ok(())
}
