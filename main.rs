struct RingBuffer<const N: usize> {
    buffer: [i32; N],
    write_index: usize,
    read_index: usize,
    is_empty: bool
}

impl<const N: usize> RingBuffer<N> {
    fn new() -> Self {
        RingBuffer {
            buffer: [0; N],
            write_index: 0,
            read_index: 0,
            is_empty: true
        }
    }

    fn increment_read_index(&mut self) {
        if self.read_index == N - 1 {
            self.read_index = 0
        }
        else {
            self.read_index += 1
        }
    }

    fn increment_write_index(&mut self) {
        if self.write_index == N - 1 {
            self.write_index = 0
        }
        else {
            self.write_index += 1
        }
    }

    fn read(&mut self) -> Option<i32> {
        //if the read index and write index are equal, we need to make sure the buffer isn't empty
        if self.read_index == self.write_index && self.is_empty == true {
            return None
        }
        //read the value from the buffer
        let value = self.buffer[self.read_index];
        //increment the read index for the next read
        self.increment_read_index();
        //if read index runs into write index, set is_empty to true to prevent further reads without a write
        if self.read_index == self.write_index {
            self.is_empty = true;
        }
        return Some(value);
    }

    fn write(&mut self, value: i32) {
        self.buffer[self.write_index] = value;
        if self.write_index == self.read_index && self.is_empty == false {
            self.increment_read_index();
        }
        self.increment_write_index();
        self.is_empty = false;
    }

}

fn main() {
    let mut rb: RingBuffer<5> = RingBuffer::new();

    println!("{}", rb.read().unwrap_or(-1));
    rb.write(1);
    println!("{}", rb.read().unwrap_or(-1));
    println!("{}", rb.read().unwrap_or(-1));
    rb.write(1);
    rb.write(2);
    rb.write(3);
    rb.write(4);
    println!("{}", rb.read().unwrap_or(-1));
    println!("{}", rb.read().unwrap_or(-1));
    rb.write(5);
    rb.write(6);
    println!("{}", rb.read().unwrap_or(-1));
    println!("{}", rb.read().unwrap_or(-1));
    println!("{}", rb.read().unwrap_or(-1));
    println!("{}", rb.read().unwrap_or(-1));
    println!("{}", rb.read().unwrap_or(-1));
    
}
