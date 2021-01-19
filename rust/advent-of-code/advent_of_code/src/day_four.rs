// Given an input, identify a value that fulfils the following criteria;
// 1. It is a six-digit number.
// 2. The value is within the range given in your puzzle input.
// 3. Two adjacent digits are the same (like 22 in 122345).
// 4. Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).

// Approach:
// Range between the two values provided
// At the point that two identical digits are identified at any point in the 6 digits, loop through the value
// For 0..6, traverse the values to the right and check they are equal to or greater than the previous index
// If the values fulfil that criteria, add the password to a Vec of successful passwords
// Get the len() of the results Vec to learn the total number of successful passwords

// Structure:
// 1. run function calls the initial range function with the inputs as a param, returning a Vec
// 2. In the range function, an if statement evaluates if there are two identical digits at the given index.
// 3. Calls an evaluator function that returns true if the value fulfils the criteria
// 4. Range function pushes value to the results array. Returns at the end of loop
// 5. Run function calculates len() and returns value in Result.

use super::error::Error;
use std::collections::HashMap;

pub fn run() -> Result<Vec<i32>, Error> {
    let mut range_checker = RangeChecker::new(234208,765869);
    range_checker.check_range_for_matches();
    let (part_1, part_2) = range_checker.get_matches();
    Ok(vec![part_1, part_2])
}

pub struct RangeChecker {
    range_vec: Vec<i32>,
    part_one_matches: Vec<i32>,
    part_two_matches: Vec<i32>
}

impl RangeMethods for RangeChecker {
    fn new(start: i32, finish: i32) -> Self {
        RangeChecker {
            range_vec: (start..finish+1).collect(),
            part_one_matches: Vec::new(),
            part_two_matches: Vec::new()
        }
    }

    fn check_range_for_matches(&mut self) {
        for val in self.range_vec.clone().into_iter() {
            let mut neighbouring_identical_nums = false;
            let mut all_ascending_num = false;
            let mut double_digit_hashmap: HashMap<u32, u32> = HashMap::new();
            let num_as_slice: Vec<u32> = val.clone().to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

            for (i, num) in num_as_slice.clone().into_iter().enumerate() {
                if i > 0 && num < num_as_slice[i - 1] {
                    break;
                } else if i == num_as_slice.len() - 1 {
                    all_ascending_num = true;
                }
            }

            for (i, num) in num_as_slice.clone().into_iter().enumerate() {
                if i != num_as_slice.len() - 1 && num == num_as_slice[i + 1] {
                    neighbouring_identical_nums = true;
                    if let Some(_entry) = double_digit_hashmap.get(&num) {
                        *double_digit_hashmap.entry(num).or_insert(1) += 1;
                    } else {
                        double_digit_hashmap.entry(num).or_insert(1);
                    }
                }
            }


            if double_digit_hashmap.values().any(|&x| x == 1) && all_ascending_num && neighbouring_identical_nums {
                    self.part_two_matches.push(val);
            }

            if all_ascending_num && neighbouring_identical_nums {
                self.part_one_matches.push(val);
            }
        }
    }
    /// Gets the results that match the given range.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::advent_of_code::day_four::*;
    /// let mut range_finder = RangeChecker::new(112233, 112234);
    /// range_finder.check_range_for_matches();
    /// let matches = range_finder.get_matches();
    ///
    /// assert_eq!(matches, (2 as i32, 2 as i32));
    /// ```
    fn get_matches(&self) -> (i32, i32) {
        (self.part_one_matches.len() as i32, self.part_two_matches.len() as i32)
    }
}

pub trait RangeMethods {
    fn new(start: i32, finish: i32) -> Self;
    fn check_range_for_matches(&mut self);
    fn get_matches(&self) -> (i32, i32);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_range_checker() {
        let range_checker = RangeChecker::new(1,10);
        // assert_eq!(range_checker.range_vec.len(), 10);
        assert_eq!(range_checker.range_vec, vec![1,2,3,4,5,6,7,8,9,10]);
    }
}