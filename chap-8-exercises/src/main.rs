fn main() {
    // Ex. 1:
    // Given a list of integers, use a vector and
    // return the mean (the average value),
    // median (when sorted, the value in the middle position),
    // and mode (the value that occurs most often;
    // (a hash map will be helpful here) of the list.

    let nums = vec![1, 5, 2, 3, 4, 5, 6];
    let mean = find_mean(&nums);
    let median = find_median(&nums);
    let mode = find_mode(&nums);
    println!("Mean: {:?}, median: {:?}, mode: {:?}", mean, median, mode);

    // Convert strings to pig latin:
    // First consonant of each word is moved to the end and "ay" is added
    // Words starting with a vowel have "hay" added to the end
    let word = "hello".to_string();
    println!("Pig says: {:?}", translate(word));
}

fn find_mean(nums: &Vec<i32>) -> f32 {
    let mut sum = 0;
    let mut count = 0;
    for num in nums {
        sum += num;
        count += 1;
    }
    (sum as f32) / (count as f32)
}

fn find_median(nums: &Vec<i32>) -> f32 {
    let mut sorted_nums = nums.to_vec();
    sorted_nums.sort();
    let len = sorted_nums.len();
    if len % 2 == 0 {
        let first = sorted_nums[len/2 - 1] as f32;
        let second = sorted_nums[len/2] as f32;
        (first + second) / 2.0
    } else {
        sorted_nums[len/2] as f32
    }
}

use std::collections::HashMap;

fn find_mode(nums: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut mode_count = 0;
    for (num, count) in map.iter() {
        if *count > mode_count {
            mode = **num;
            mode_count = *count;
        }
    }
    mode
}

fn translate(mut word: String) -> String {
    let first = word.remove(0);
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    if vowels.iter().any(|&v| v == first) {
        word.push('-');
        word.push_str("hay");
    } else {
        word.push('-');
        word.push(first);
        word.push_str("ay");
    }
    word
}
