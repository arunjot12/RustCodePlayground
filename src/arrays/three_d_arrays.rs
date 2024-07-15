pub fn three_d_arrays() {
    let array: [[[i32; 3]; 3]; 2] = 
    [
        [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ],
        [
            [10, 11, 12],
            [12, 13, 14],
            [15, 16, 17],
        ],
    ];
    for first in array{
        for second in first{
            for third in second{
                print!("{} ",third)
            }
            println!()
        }
        println!()
    }
}
