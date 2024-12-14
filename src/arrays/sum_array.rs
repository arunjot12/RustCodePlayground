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
pub fn two_sum_array() {
    let nums = [1, 2, 3, 4, 5];
    let target = 9;
    for i in nums {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                println!("first {} second {}", nums[i], nums[i + 1]);
            }
        }
    }
}

// Print the sum in between two random address
pub fn two_sum() {
    let nums = [1, 2, 4, 7, 9];
    let target = 11;

    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                println!("Pair found: {} and {}", nums[i], nums[j]);
                return; // Exit after finding the first pair
            }
        }
    }
    println!("No pair found that sums to the target");
}

// Print the sum of the vec
pub fn two_vec_sum(){
    let data:Vec<i32> = vec![14,15,6];
    let target :i32 = 21;
    let mut output: Vec<i32> = Vec::new();
    for i in 0..data.len(){
        for j in i+1..data.len(){ 
            if data[i] + data[j] == target{
                println!("Found elements at indices {} and {} with values {} and {}", i, j, data[i], data[j]);
                output.push((i).try_into().unwrap());
                output.push((j).try_into().unwrap());
            }
        }
    }
    println!("output {:?}",output);
}

