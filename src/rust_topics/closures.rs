pub fn show_closures() {
    println!("--- Closures Demo ---");
    basic_closure();
    immutable_borrow();
    mutable_borrow();
    ownership_moves();
}

fn basic_closure() {
    let a = 10;
    let b = |n| n + a;
    println!("Basic closure capturing 'a': {:?}", b(10));
}

fn immutable_borrow() {
    let data = vec![1, 2, 3];
    let print = || println!("Immutable borrow of data: {:?}", data);
    print();
}

fn mutable_borrow() {
    let mut data = vec![1, 2, 3];
    let mut push = || {
        data.push(9);
    };
    push();
    println!("Mutable borrow after push: {:?}", data);
}

fn ownership_moves() {
    let data = vec![1, 2, 3];
    let new_data = move || {
        println!("Ownership moved into closure, dropping data: {:?}", data);
        drop(data);
    };
    new_data();
}
