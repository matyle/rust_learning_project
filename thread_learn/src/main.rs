// fn main() {
//     let mut handles = vec![];
//     for i in 0..10 {
//         handles.push(thread::spawn(move || {
//             thread::sleep(Duration::from_millis(250));
//             println!("Hello from thread {}", i);
//         }));
//     }

//     let mut completed_threads = 0;
//     for handle in handles {
//         // TODO: a struct is returned from thread::spawn, can you use it?
//         // handle.join().unwrap();
//         match handle.join() {
//             Ok(_) => completed_threads += 1,
//             Err(_) => println!("join has a error"),
//         }
//     }

//     if completed_threads != 10 {
//         panic!("Oh no! All the spawned threads did not finish!");
//     }
// }
//
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: 0 });
    let mut handles = vec![];
    // let mtx = Mutex::new(0);
    let mtx = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let mtx = Arc::clone(&mtx);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            // mtx.lock();
            let mut completed = mtx.lock().unwrap();
            completed.jobs_completed += 1;
            // status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }
    // for handle in handles {
    //     handle.join().unwrap();
    //     // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
    //     // interesting in the output? Do you have to 'join' on all the handles?
    //     println!("jobs completed {:?}", mtx.lock().unwrap().jobs_completed)
    // }
    handles.into_iter().for_each(|x| {
        x.join().unwrap();
        println!("jobs completed {:?}", mtx.lock().unwrap().jobs_completed)
    })
}
