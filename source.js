function heavySort(arr) {
    if (arr.length === 0) {
        return arr;
    }

    const minValue = Math.min(...arr);
    const maxValue = Math.max(...arr);
    
    // Step 1: Create an object to act as the bucket
    const buckets = {};
    
    // Step 2: Push numbers into the bucket
    for (const num of arr) {
        if (!(num in buckets)) {
            buckets[num] = 0;
        }
        buckets[num] += 1;
    }
    
    // Step 3: Create the sorted array
    const sortedArr = [];
    for (let num = minValue; num <= maxValue; num++) {
        if (num in buckets) {
            sortedArr.push(...Array(buckets[num]).fill(num));
        }
    }
    
    return sortedArr;
}

// Example usage
const arr = [1, 3, 6, 7];
const sortedArr = heavySort(arr);
console.log("Sorted array:", sortedArr);
