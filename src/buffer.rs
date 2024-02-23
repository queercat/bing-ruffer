pub struct Buffer<T> {
    data: Vec<T>,
}
pub trait Write<T> {
    fn write(&mut self, data: T);
}

pub trait Read<T> {
    fn read(&mut self) -> Option<T>;
}

impl<T> Buffer<T> {
    pub fn new() -> Self {
        Buffer { data: Vec::new() }
    }
}

impl<T> Write<T> for Buffer<T> {
    fn write(&mut self, data: T) {
        self.data.push(data);
    }
}

impl<T> Read<T> for Buffer<T> {
    fn read(&mut self) -> Option<T> {
        self.data.pop()
    }
}
