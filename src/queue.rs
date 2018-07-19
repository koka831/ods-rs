

trait Queue<T> {
    // append x to Queue
    fn add(&mut self, x: T);
    // remove Queue according to in-out rule
    fn remove(&mut self) -> Option<T>;
}


trait Deque<T> {
    fn add_first(&mut self, x: T);
    fn add_last(&mut self, x: T);
    fn remove_first(&mut self) -> T;
    fn remove_last(&mut self) -> T;
}

struct FIFO<T> {
    queue: Vec<T>,
    len: usize,
}

impl<T> Queue<T> for FIFO<T> {
    fn add(&mut self, x: T) {
        self.queue.insert(self.len - 1, x);
        self.len += 1;
    }

    fn remove(&mut self) -> Option<T> {
        self.len -= 1;
        self.queue.pop()
    }
}

struct Stack<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> for Stack<T> {
    fn add(&mut self, x: T) {
        // push x to last
        self.queue.push(x);
    }

    fn remove(&mut self) -> Option<T> {
        // return last
        self.queue.pop()
    }
}
