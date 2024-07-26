use std::collections::HashMap;

fn heavy_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.is_empty() {
        return arr;
    }

    let min_value = *arr.iter().min().unwrap();
    let max_value = *arr.iter().max().unwrap();
    
    let mut buckets: HashMap<i32, usize> = HashMap::new();
    
    for &num in &arr {
        *buckets.entry(num).or_insert(0) += 1;
    }
    
    let mut sorted_arr = Vec::new();
    for num in min_value..=max_value {
        if let Some(&count) = buckets.get(&num) {
            sorted_arr.extend(vec![num; count]);
        }
    }
    
    sorted_arr
}

fn main() {
    let arr = vec![1, 3, 6, 7];
    let sorted_arr = heavy_sort(arr);
    println!("Sorted Array: {:?}", sorted_arr);
}
