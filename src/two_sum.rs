use std::convert::TryInto;

pub fn run(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index1 = 0;
    let mut index2: Option<usize> = None;

    let mut sort_nums: Vec<i32> = nums.clone();
    sort_nums.sort();

    while index2.is_none() {
        let search_target = target - sort_nums.get(index1).expect("index1 out of bounds");
        index2 = match sort_nums.binary_search(&search_target) {
            Ok(i) => Some(i),
            Err(_) => {
                index1 += 1;
                None 
            },
        };
    }
    let result1 = nums.iter().position(|x| *x == sort_nums[index1]);
    let mut result2 = nums.iter().position(|x| *x == sort_nums[index2.unwrap()]);

    if result2 == result1 {
        result2 = nums[(result1.unwrap() + 1)..].iter().position(|x| *x == sort_nums[index2.unwrap()]);
        result2 = Some(result2.unwrap() + (result1.unwrap() + 1));
    }

    vec![result1.unwrap().try_into().unwrap(), result2.unwrap().try_into().unwrap()]
}

