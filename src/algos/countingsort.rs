pub fn countingsort(vector: &mut Vec<i32>) {
    let mut counts = Vec::<usize>::new();
    let mut min = *vector.iter().min().unwrap();

    if min < 0 {
        min = -min;
    }

    for i in 0..vector.len() {
        while counts.len() < (vector[i] - min + 1) as usize {
            counts.push(0);
        }

        counts[(vector[i] - min) as usize] += 1;
    }

    let mut index = 0;

    for i in 0..counts.len() {
        while counts[i] > 0 {
            vector[index] = i as i32 + min;
            counts[i] -= 1;
            index += 1;
        }
    }
}
