# Copyright © Bùi Nguyễn Tấn Sang (EndermanPC) 2024

def heavy_sort(arr):
    if not arr:
        return arr

    min_value = min(arr)
    max_value = max(arr)
    
    buckets = {}
    
    for num in arr:
        if num not in buckets:
            buckets[num] = 0
        buckets[num] += 1
    
    sorted_arr = []
    for num in range(min_value, max_value + 1):
        if num in buckets:
            sorted_arr.extend([num] * buckets[num])
    
    return sorted_arr

arr = [1, 3, 6, 7]
sorted_arr = heavy_sort(arr)
print("Sorted Array:", sorted_arr)
