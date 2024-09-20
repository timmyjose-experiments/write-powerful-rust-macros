// our custom trait

trait Hello {
    fn hello(&self);
}

// blanket implementation
impl<T: Copy> Hello for T {
    fn hello(&self) {
        println!("Hello!");
    }
}

fn main() {
    '1'.hello();
    true.hello();
    1.2345.hello();
    'x'.hello();
    (100, false).hello();
}
