pub fn pairs(nums: Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    let mut i: usize = 0;
    let mut j: usize;

    while i < nums.len() {
        j = i + 1;
        while j < nums.len() {
            if nums[j] == nums[i] {
                count += 1;
            }
            j += 1;
        }
        i += 1;
    }
    count
}