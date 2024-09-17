use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

const N_SECONDS: u64 = 5; // Время выполнения программы в секундах

fn main() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    // Запуск потока для отправки значений
    let tx_thread = thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Запуск потока для чтения значений
    let rx_thread = {
        let rx = Arc::clone(&rx);
        thread::spawn(move || {
            let start_time = std::time::Instant::now();
            while start_time.elapsed().as_secs() < N_SECONDS {
                match rx.lock().unwrap().recv_timeout(Duration::from_millis(100)) {
                    Ok(value) => println!("Received: {}", value),
                    Err(_) => continue,
                }
            }
            println!("Time limit reached. Exiting...");
        })
    };

    tx_thread.join().unwrap();
    rx_thread.join().unwrap();
}
