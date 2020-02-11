use crate::args;

pub fn bubblesort(vector: &mut Vec<i32>, state_value: &mut args::StateValue) {
    // If no two neighboring elements were swapped, then this means 
    // that all elements are sorted and sorting can be completed.
    let mut swap_flag = true;
    state_value.memory_accesses += 1;

    // After each pass, the last element is guaranteed to be the largest
    // and stands in its place, so there is no point in checking it.
    // To implement this idea, we will each time reduce the value of this
    // variable by 1 in order to shorten the range of the for-cycle by 1
    // last element.
    let mut length = vector.len();
    state_value.memory_accesses += 1;

    while swap_flag {
        swap_flag = false;

        for i in 1..length {
            if vector[i - 1] > vector[i] {
                vector.swap(i - 1, i);
                swap_flag = true;
                state_value.memory_accesses += 4;
            }

            state_value.comparisons += 1;
        }

        length -= 1;
        state_value.comparisons += 1;
        state_value.memory_accesses += 2;
    }
}
