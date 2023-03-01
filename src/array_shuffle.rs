/// Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].
/// Return the array in the form [x1,y1,x2,y2,...,xn,yn].
pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut i = 0;
 
    let nums_x = &nums[0..=(n-1)];
    let nums_y = &nums[n..];
    
    let mut result: Vec<i32> = Vec::with_capacity(2*n);
 
    while i != n {
        result.push(nums_x[i]);
        result.push(nums_y[i]);
        i += 1;
    }
 
    result
}