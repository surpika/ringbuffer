use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

struct RingBuffer<const N: usize> {
    // Use a single mutex to protect the entire state
    state: Mutex<RingBufferState<N>>
}

struct RingBufferState<const N: usize> {
    buffer: [i32; N],
    write_index: usize,
    read_index: usize,
    is_empty: bool
}

impl<const N: usize> RingBuffer<N> {
    fn new() -> Self {
        RingBuffer {
            state: Mutex::new(RingBufferState {
                buffer: [0; N],
                write_index: 0,
                read_index: 0,
                is_empty: true
            })
        }
    }
    
    fn read(&self) -> Option<i32> {
        let mut state = self.state.lock().unwrap();
        
        if state.read_index == state.write_index && state.is_empty {
            return None;
        }
        
        let value = state.buffer[state.read_index];
        
        // Increment read index
        if state.read_index == N - 1 {
            state.read_index = 0;
        } else {
            state.read_index += 1;
        }
        
        // Check if read caught up to write
        if state.read_index == state.write_index {
            state.is_empty = true;
        }
        
        Some(value)
    }
    
    fn write(&self, value: i32) {
        let mut state = self.state.lock().unwrap();
        let write_idx = state.write_index;
        state.buffer[write_idx] = value;
        
        if state.write_index == state.read_index && !state.is_empty {
            // Increment read index - buffer is full
            if state.read_index == N - 1 {
                state.read_index = 0;
            } else {
                state.read_index += 1;
            }
        }
        
        // Increment write index
        if state.write_index == N - 1 {
            state.write_index = 0;
        } else {
            state.write_index += 1;
        }
        
        state.is_empty = false;
    }
}

fn main() {
    let rb = Arc::new(RingBuffer::<5>::new());
    
    // Multiple producer threads
    let mut producers = vec![];
    for id in 0..3 {
        let rb_clone = Arc::clone(&rb);
        let producer = thread::spawn(move || {
            for i in 1..=5 {
                let value = i + (id * 100); // Create unique values per producer
                rb_clone.write(value);
                println!("Producer {}: wrote {}", id, value);
                //thread::sleep(Duration::from_millis(50 + (id as u64) * 10));
            }
        });
        producers.push(producer);
    }
    
    // Multiple consumer threads
    let mut consumers = vec![];
    for id in 0..2 {
        let rb_clone = Arc::clone(&rb);
        let consumer = thread::spawn(move || {
            for _ in 1..=8 {
                match rb_clone.read() {
                    Some(value) => println!("Consumer {}: read {}", id, value),
                    None => println!("Consumer {}: buffer empty", id),
                }
                //thread::sleep(Duration::from_millis(100 + (id as u64) * 20));
            }
        });
        consumers.push(consumer);
    }
    
    // Wait for all producers to finish
    for producer in producers {
        producer.join().unwrap();
    }
    
    // Wait for all consumers to finish
    for consumer in consumers {
        consumer.join().unwrap();
    }
    
    // Check final state
    println!("\nFinal buffer state:");
    while let Some(value) = rb.read() {
        println!("Remaining: {}", value);
    }
}