#![allow(dead_code)]

mod two_sum;
mod two_sum_sorted;
mod running_sum;
mod defang_ip;
mod good_pairs;
mod array_shuffle;

fn main(){
    println!("run cargo tests to test modules");
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn two_sum_test() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let results = two_sum::run(nums, target);
        assert_eq!(results, vec![0, 1]);

        let nums = vec![0, 3, 0, 1];
        let target = 0;

        let results = two_sum::run(nums, target);
        assert_eq!(results, vec![0, 2]);

        let nums = vec![6, 4, 3, 15];
        let target = 18;

        let results = two_sum::run(nums, target);
        assert_eq!(results, vec![2, 3]);
    }

    #[test]
    fn two_sum_sorted_test() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let results = two_sum_sorted::run(nums, target);
        assert_eq!(results, vec![1, 2]);

        let nums = vec![1, 1, 7, 8];
        let target = 2;

        let results = two_sum_sorted::run(nums, target);
        assert_eq!(results, vec![1, 2]);

        let nums = vec![2, 7, 11, 15];
        let target = 26;

        let results = two_sum_sorted::run(nums, target);
        assert_eq!(results, vec![3, 4]);
    }

    #[test]
    fn running_sum_test() {
        let nums = vec![1,2,3,4];
        let results = running_sum::run(nums);
        assert_eq!(results, vec![1,3,6,10]);

        let nums = vec![1,1,1,1,1];
        let results = running_sum::run(nums);
        assert_eq!(results, vec![1,2,3,4,5]);

        let nums = vec![3,1,2,10,1];
        let results = running_sum::run(nums);
        assert_eq!(results, vec![3,4,6,16,17]);        
    }

    #[test]
    fn defang_ip_test() {
        let address = String::from("1.1.1.1");
        let result = defang_ip::run(address);
        assert_eq!(result, "1[.]1[.]1[.]1");

        let address = String::from("255.100.50.0");
        let result = defang_ip::run(address);
        assert_eq!(result, "255[.]100[.]50[.]0");

    }

    #[test]
    fn good_pair_test() {
        let nums = vec![1,2,3,1,1,3];
        let expected = 4;
        assert_eq!(good_pairs::pairs(nums), expected);
 
        let nums = vec![1,1,1,1];
        let expected = 6;
        assert_eq!(good_pairs::pairs(nums), expected);
 
        let nums = vec![1,2,3];
        let expected = 0;
        assert_eq!(good_pairs::pairs(nums), expected);
    }

    #[test]
    fn array_shuffle_test() {
        let nums = vec![2,5,1,3,4,7];
        let n = 3;
        let result = array_shuffle::shuffle(nums, n);
        assert_eq!(result, vec![2,3,5,4,1,7])
    }
}