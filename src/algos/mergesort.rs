pub fn mergesort(vector: Vec<i32>) -> Vec<i32>  {
    // A vector of one or zero element is sorted by definition,
    // so this is a condition for exiting recursion.
    if vector.len() <= 1 {
        return vector;
    }

    let mut left = Vec::new();
    let mut rigth = Vec::new();

    for i in 0..vector.len() {
        if i < vector.len() / 2 {
            left.push(vector[i]);
        } else {
            rigth.push(vector[i]);
        }
    }

    // Recursively sort both subvectors.
    left = mergesort(left);
    rigth = mergesort(rigth);

    merge(left, rigth)
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut merged = Vec::new();
    // The for loop defines a range of indices from zero to the length
    // of the combined vector, and we need variables that indicate the
    // indices of the left and right vectors.
    let mut i = 0;
    let mut j = 0;

    for _ in 0..left.len() + right.len() {
        if i < left.len() && j < right.len() && left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else if i < left.len() && j < right.len() && left[i] > right[j] {
            merged.push(right[j]);
            j += 1;
        } else if j >= right.len() {
            merged.push(left[i]);
            i += 1;
        } else if i >= left.len() {
            merged.push(right[j]);
            j += 1;
        }
    }
    
    merged
}
