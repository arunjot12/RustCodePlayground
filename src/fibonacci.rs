pub fn fibonacci(){
    let mut fib = [1;15];
    for i in 2..fib.len(){
        fib[i]= fib[i-2] + fib[i-1];
    }
    for i in 0..fib.len(){
    println!("{}",fib[i] );
    }
}