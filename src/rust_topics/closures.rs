fn _closure() {
    let a = 10;
    let b = |n| n + a ;
    println!(" The b is {:?}",b(10));
}

// Because a  function can't do this

// fn add(n:i32) -> i32{
//     n + x
// }

// Immutable borrow 

fn _immutable_borrow(){
    let data = vec![1,2,3];
    let print = || println!("{:?}",data);
    print();
    // This means we have reference this using Fn methods of the closure
}

//   Mutable borrow through the closure 

fn _mutable_borrow(){
    let mut data = vec![1,2,3];
    let mut push = || {
        data.push(9);
    };
    push();
}