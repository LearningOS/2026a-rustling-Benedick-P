// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.
use std::sync::{Arc, Mutex}; // 1. 引入 Mutex
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 2. 用 Mutex 把 JobStatus 包起来，再用 Arc 共享
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 3. 获取锁，才能修改内部数据
            let mut status_guard = status_shared.lock().unwrap();
            status_guard.jobs_completed += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
        // 4. 打印时也要获取锁
        let status_guard = status.lock().unwrap();
        println!("jobs completed {}", status_guard.jobs_completed);
    }
}