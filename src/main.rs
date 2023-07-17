use std::collections::VecDeque;
// use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let queue= Arc::new(Mutex::new(VecDeque::new()));

    let cloned = Arc::clone(&queue);
    let cloned2 = Arc::clone(&queue);

    {
        let mut q = queue.lock().unwrap();
        q.push_back("111111".to_string());
        q.push_back("2222222".to_string());
    }
    // キューを監視して要素を取り出す
    let thread = spawn(move || {
        println!("start deque thread");
        loop {
            let mut q = cloned.lock().unwrap();
            if let Some(element) = q.front() {
                println!("pop {}", *element);
                q.pop_front();
            }
        }
    });

    // キューに要素を追加する（2秒ごと）
    let elements: Vec<String> = vec!["AAA".to_string(), "BBB".to_string(), "CCC".to_string(), "DDD".to_string(),
                                     "EEE".to_string(), "FFF".to_string(), "GGG".to_string(), "HHH".to_string(),
                                     "III".to_string(), "JJJ".to_string()];
    let thread2 = spawn(move || {
        println!("start enqueue thread");
        for i in elements {
            {
                let mut q = cloned2.lock().unwrap();
                let br = &i;
                q.push_back(br.to_string());
                println!("push {}", br);
            }
            sleep(Duration::from_secs(2));
        }
    });

    thread.join().unwrap();
    thread2.join().unwrap();
}

