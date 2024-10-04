use std::thread;
use std::sync::mpsc;

fn main() {
    let n: usize = 10;              // Количество чисел
    let num_threads: usize = 5;     // Количество потоков (<=n/2)
    let numbers: Vec<usize> = (1..=n).collect();

    // Количество чисел, которые обработает каждый поток
    let chunk_size: usize = (n as f32 / num_threads as f32).ceil() as usize;

    let mut handles = vec![];

    let (tx, rx) = mpsc::channel();

    for i in 0..num_threads {
        let tx = tx.clone();

        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, n);

        let chunk = numbers[start..end].to_vec();

        let handle = thread::spawn(move || {
            let mut sum: usize = 0;
            for num in chunk {
                sum += num * num;
            }
            tx.send(sum).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut result: usize = 0;
    for _ in 0..num_threads {
        result += rx.recv().unwrap();
    }

    println!("Sum of squares of numbers from 1 to {}: {}", n, result);
}

