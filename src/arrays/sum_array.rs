// Addition of the array
pub fn sum_of_array() {
    let array = [5, 2, 3, 4, 5];
    let mut sum = 0;
    for i in array {
        sum = sum + i;
    }
    println!("Total sum of 1D array is {}", sum);
}

// Print the two sum of the target
pub fn two_sum() {
    let nums = [1, 2, 3, 4, 5];
    let target = 9;
    for i in nums {
        for j in (i+1)..nums.len(){
        if nums[i] + nums[j] == target  {
            println!("first {} second {}", nums[i], nums[i + 1]);
        }
    }
    }
}
