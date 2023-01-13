mod bound;

fn main() {
    println!("Hello, world!");
}

trait AIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: i32,
}

impl AIterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

trait GIterator<T> {
    fn prev(&mut self) -> Option<T>;
}

impl GIterator<i32> for Counter {
    fn prev(&mut self) -> Option<i32> {
        if self.count == 5 {
            self.count -= 1;
            Some(self.count)
        } else {
            None
        }
    }
}
