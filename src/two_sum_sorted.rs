use std::convert::TryInto;

pub fn run(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index1: usize = 0;
    let mut index2: Option<usize> = None;

    while index2.is_none() {
        let search_target = target - nums.get(index1).expect("index1 out of bounds");
        index2 = match nums.binary_search(&search_target) {
            Ok(i) => Some(i),
            Err(_) => {
                index1 += 1;
                None 
            },
        };
    }
    
    let index1: i32 = index1.try_into().unwrap();
    let index2: i32 = index2.unwrap().try_into().unwrap();
    vec![(index1 + 1), (index2 + 1)]
}

