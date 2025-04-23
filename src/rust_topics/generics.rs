// Example of Generics 

fn generic<T>(a:char, b:T, c:T) -> T {
    if a == 'a'{
        b
    }
    else
    {
        c
    }
}

pub fn use_of_generic(){
    let a = generic::<f64>('a',5.5,1.2);
    let b = generic::<i32>('a',5,1);
    println!("{},{}",a,b);
}