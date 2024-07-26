#include <iostream>
#include <vector>
#include <unordered_map>
#include <algorithm>

std::vector<int> heavy_sort(const std::vector<int>& arr) {
    if (arr.empty()) {
        return arr;
    }

    int min_value = *std::min_element(arr.begin(), arr.end());
    int max_value = *std::max_element(arr.begin(), arr.end());
    
    // Step 1: Create a map to act as the bucket
    std::unordered_map<int, int> buckets;
    
    // Step 2: Push numbers into the bucket
    for (int num : arr) {
        buckets[num]++;
    }
    
    // Step 3: Create the sorted array
    std::vector<int> sorted_arr;
    for (int num = min_value; num <= max_value; ++num) {
        if (buckets.find(num) != buckets.end()) {
            sorted_arr.insert(sorted_arr.end(), buckets[num], num);
        }
    }
    
    return sorted_arr;
}

// Example usage
int main() {
    std::vector<int> arr = {1, 3, 6, 7};
    std::vector<int> sorted_arr = heavy_sort(arr);
    std::cout << "Sorted array: ";
    for (int num : sorted_arr) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
    return 0;
}
