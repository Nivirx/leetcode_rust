pub fn run(nums: Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::with_capacity(nums.len());

    let mut i = 1;
    while i <= nums.len() {
        results.push(nums[0..i].iter().sum());
        i += 1;
    }
    
    results
}