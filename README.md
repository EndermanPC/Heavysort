# Heavy Sort Algorithm

## Introduction
The Heavy Sort algorithm simulates the process of pushing numbers into a water bucket, where they settle at positions corresponding to their values. When the numbers sink or float, they are sorted in ascending order.

## How It Works
1. **Initialization**:
   - Find the minimum and maximum values in the input array.
   - Create a dictionary structure to represent the bucket, where each number value is a position in the bucket.

2. **Pushing Numbers into the Bucket**:
   - Iterate through each number in the input array.
   - For each number, place it in the corresponding position in the dictionary, and count its occurrences.

3. **Creating the Sorted Array**:
   - Iterate through all values from the minimum to the maximum.
   - If the value exists in the dictionary, add its occurrences to the result array.

## Python Code

```python
def heavy_sort(arr):
    if not arr:
        return arr

    min_value = min(arr)
    max_value = max(arr)
    
    # Step 1: Create a dictionary to act as the bucket
    buckets = {}
    
    # Step 2: Push numbers into the bucket
    for num in arr:
        if num not in buckets:
            buckets[num] = 0
        buckets[num] += 1
    
    # Step 3: Create the sorted array
    sorted_arr = []
    for num in range(min_value, max_value + 1):
        if num in buckets:
            sorted_arr.extend([num] * buckets[num])
    
    return sorted_arr

# Example usage
arr = [1, 3, 6, 7]
sorted_arr = heavy_sort(arr)
print("Sorted array:", sorted_arr)
```

## Complexity Analysis
**Space**:
- Uses a dictionary to store elements by their values. In the worst case, this requires O(N) space if all elements are unique.

**Time**:
- **Finding min and max values**: O(N).
- **Pushing numbers into the bucket (dictionary)**: O(N).
- **Creating the sorted array from the dictionary**: O(N + M), where M is the range between the smallest and largest numbers.

## Comparison with Other Algorithms
- **Bubble Sort, Selection Sort, Insertion Sort**: O(N^2) time complexity in the worst case, O(1) space complexity.
- **Merge Sort**: O(N log N) time complexity in all cases, O(N) space complexity.
- **Quick Sort**: Average O(N log N) time complexity, worst case O(N^2), O(log N) space complexity.
- **Counting Sort**: O(N + K) time complexity, O(K) space complexity.
- **Radix Sort**: O(N * k) time complexity, O(N + K) space complexity.

## Performance on test data:
- **Array of size 10^6, randomly generated numbers from 1 to 100**: runs in `0.113055s`.
- **Array of size 10^5, randomly generated numbers from 0 to 100**: runs in `0.016916s`.

## Conclusion
The Heavy Sort algorithm is an innovative and intuitive approach based on the idea of weight and the sinking/floating of numbers. While it may not be space-efficient when the largest value is too large, using a dictionary optimizes memory usage and keeps the time complexity acceptable. This makes the algorithm a unique and effective solution in many practical scenarios.

## Contact
For any questions or contributions, please reach out to me at [endermatday@gmail.com].

## License
This project is licensed under the Apache 2.0 License.
