use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct RingBuffer<const N: usize> {
    buffer: Mutex<[i32; N]>,
    write_index: Mutex<usize>,
    read_index: Mutex<usize>,
    is_empty: Mutex<bool>
}

impl<const N: usize> RingBuffer<N> {
    fn new() -> Self {
        RingBuffer {
            buffer: Mutex::new([0; N]),  // Fixed: wrap with Mutex::new()
            write_index: Mutex::new(0),
            read_index: Mutex::new(0),
            is_empty: Mutex::new(true)
        }
    }
    
    fn increment_read_index(&self) {
        let mut read_idx = self.read_index.lock().unwrap();
        if *read_idx == N - 1 {
            *read_idx = 0;
        } else {
            *read_idx += 1;
        }
    }
    
    fn increment_write_index(&self) {
        let mut write_idx = self.write_index.lock().unwrap();
        if *write_idx == N - 1 {
            *write_idx = 0;
        } else {
            *write_idx += 1;
        }
    }
    
    fn read(&self) -> Option<i32> {
        let read_idx = *self.read_index.lock().unwrap();
        let write_idx = *self.write_index.lock().unwrap();
        let is_empty = *self.is_empty.lock().unwrap();
        
        // If the read index and write index are equal, check if buffer is empty
        if read_idx == write_idx && is_empty {
            return None;
        }
        
        // Read the value from the buffer
        let value = self.buffer.lock().unwrap()[read_idx];
        
        // Increment the read index
        self.increment_read_index();
        
        // Check if read caught up to write
        let new_read_idx = *self.read_index.lock().unwrap();
        if new_read_idx == write_idx {
            *self.is_empty.lock().unwrap() = true;
        }
        
        Some(value)
    }
    
    fn write(&self, value: i32) {
        let write_idx = *self.write_index.lock().unwrap();
        let read_idx = *self.read_index.lock().unwrap();
        let is_empty = *self.is_empty.lock().unwrap();
        
        // Write the value
        self.buffer.lock().unwrap()[write_idx] = value;
        
        // If write index caught up to read index
        if write_idx == read_idx && !is_empty {
            self.increment_read_index();
        }
        
        self.increment_write_index();
        *self.is_empty.lock().unwrap() = false;
    }
}

fn main() {
    // Create a RingBuffer shared between threads using Arc
    let ring_buffer = Arc::new(RingBuffer::<10>::new());
    
    // Clone Arc for the producer thread
    let producer_rb = Arc::clone(&ring_buffer);
    let producer = thread::spawn(move || {
        for i in 1..=20 {
            producer_rb.write(i);
            println!("Produced: {}", i);
            thread::sleep(Duration::from_millis(100)); // Slow down a bit for demonstration
        }
    });
    
    // Clone Arc for the consumer thread
    let consumer_rb = Arc::clone(&ring_buffer);
    let consumer = thread::spawn(move || {
        for _ in 1..=20 {
            match consumer_rb.read() {
                Some(value) => println!("Consumed: {}", value),
                None => println!("Buffer empty, waiting..."),
            }
            thread::sleep(Duration::from_millis(150)); // Read slightly slower than write
        }
    });
    
    // Wait for both threads to complete
    producer.join().unwrap();
    consumer.join().unwrap();
}