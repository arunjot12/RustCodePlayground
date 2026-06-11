use core::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

pub fn use_smart_pointers() {
    let a = MyBox::new(String::new());
    let b = MyBox::new(50);
    let c = *b;
}
