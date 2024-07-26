# Copyright © Bùi Nguyễn Tấn Sang (EndermanPC) 2024

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
    start_range = (min_value // 10) * 10
    end_range = ((max_value // 10) + 1) * 10
    
    sorted_arr = []
    for num in range(start_range, end_range):
        if num in buckets:
            sorted_arr.extend([num] * buckets[num])
    
    return sorted_arr

# Example usage
arr = [1, 3, 6, 7]
sorted_arr = heavy_sort(arr)
print("Sorted array:", sorted_arr)
