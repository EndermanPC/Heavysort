// Copyright © Bùi Nguyễn Tấn Sang (EndermanPC) 2024

function heavySort(arr) {
    if (arr.length === 0) {
        return arr;
    }

    const minValue = Math.min(...arr);
    const maxValue = Math.max(...arr);
    
    // Step 1: Create a Map to act as the bucket
    const buckets = new Map();
    
    // Step 2: Push numbers into the bucket
    for (const num of arr) {
        if (!buckets.has(num)) {
            buckets.set(num, 0);
        }
        buckets.set(num, buckets.get(num) + 1);
    }
    
    // Step 3: Create the sorted array
    const sortedArr = [];
    for (let num = minValue; num <= maxValue; num++) {
        if (buckets.has(num)) {
            sortedArr.push(...Array(buckets.get(num)).fill(num));
        }
    }
    
    return sortedArr;
}

// Example usage
const arr = [1, 3, 6, 7];
const sortedArr = heavySort(arr);
console.log("Sorted array:", sortedArr);
