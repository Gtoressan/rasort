use crate::args;

pub fn mergesort(vector: Vec<i32>, state_value: &mut args::StateValue) -> Vec<i32>  {
    // A vector of one or zero element is sorted by definition,
    // so this is a condition for exiting recursion.
    if vector.len() <= 1 {
        return vector;
    }
    
    state_value.comparisons += 1;

    let mut left = Vec::new();
    let mut rigth = Vec::new();
    state_value.memory_accesses += 2;

    for i in 0..vector.len() {
        if i < vector.len() / 2 {
            left.push(vector[i]);
        } else {
            rigth.push(vector[i]);
        }

        state_value.comparisons += 1;
        state_value.memory_accesses += 1;
    }

    // Recursively sort both subvectors.
    left = mergesort(left, state_value);
    rigth = mergesort(rigth, state_value);
    state_value.memory_accesses += 2;

    merge(left, rigth, state_value)
}

fn merge(left: Vec<i32>, right: Vec<i32>, state_value: &mut args::StateValue) -> Vec<i32> {
    let mut merged = Vec::new();
    // The for loop defines a range of indices from zero to the length
    // of the combined vector, and we need variables that indicate the
    // indices of the left and right vectors.
    let mut i = 0;
    let mut j = 0;
    state_value.memory_accesses += 3;

    for _ in 0..left.len() + right.len() {
        if i < left.len() && j < right.len() && left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
            state_value.comparisons += 3;
            state_value.memory_accesses += 2;
        } else if i < left.len() && j < right.len() && left[i] > right[j] {
            merged.push(right[j]);
            j += 1;
            state_value.comparisons += 6;
            state_value.memory_accesses += 2;
        } else if j >= right.len() {
            merged.push(left[i]);
            i += 1;
            state_value.comparisons += 7;
            state_value.memory_accesses += 2;
        } else {
            merged.push(right[j]);
            j += 1;
            state_value.comparisons += 7;
            state_value.memory_accesses += 2;
        }
    }
    
    merged
}
