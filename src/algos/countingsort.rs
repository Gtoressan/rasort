use crate::args;

pub fn countingsort(vector: &mut Vec<i32>, state_value: &mut args::StateValue) {
    let mut counts = Vec::<usize>::new();
    let mut min = *vector.iter().min().unwrap();
    match vector.iter().position(|x| *x == min) {
        Some(index) => {
            state_value.comparisons += index as u128;
            state_value.memory_accesses += (2 + index) as u128;
        },
        None => {
            state_value.memory_accesses += 2;
        }
    };

    if min < 0 {
        min = -min;
        state_value.memory_accesses += 1;
    }

    state_value.comparisons += 1;

    for i in 0..vector.len() {
        while counts.len() < (vector[i] - min + 1) as usize {
            counts.push(0);
            state_value.comparisons += 1;
            state_value.memory_accesses += 1;
        }

        counts[(vector[i] - min) as usize] += 1;
        state_value.memory_accesses += 1;
    }

    let mut index = 0;
    state_value.memory_accesses += 1;

    for i in 0..counts.len() {
        while counts[i] > 0 {
            vector[index] = i as i32 + min;
            counts[i] -= 1;
            index += 1;
            state_value.comparisons += 1;
            state_value.memory_accesses += 3;
        }
    }
}
