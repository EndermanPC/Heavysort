// Copyright © Bùi Nguyễn Tấn Sang (EndermanPC) 2024

use std::collections::HashMap;

fn heavy_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.is_empty() {
        return arr;
    }

    let min_value = *arr.iter().min().unwrap();
    let max_value = *arr.iter().max().unwrap();

    // Step 1: Create a map to act as the bucket
    let mut buckets: HashMap<i32, usize> = HashMap::new();

    // Step 2: Push numbers into the bucket
    for &num in &arr {
        *buckets.entry(num).or_insert(0) += 1;
    }

    // Step 3: Create the sorted array
    let mut sorted_arr = Vec::new();
    for num in min_value..=max_value {
        if let Some(&count) = buckets.get(&num) {
            sorted_arr.extend(vec![num; count]);
        }
    }
    
    sorted_arr
}

// Example usage
fn main() {
    let arr = vec![1, 3, 6, 7];
    let sorted_arr = heavy_sort(arr);
    println!("Sorted Array: {:?}", sorted_arr);
}
