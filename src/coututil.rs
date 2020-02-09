pub fn print_vector(vector: &Vec<i32>, columns: i32) {
    let mut index = 0;

    for i in vector {
        print!("{}\t", i);
        index += 1;

        if index >= columns {
            println!();
            index = 0;
        }
    }

    println!();
}

pub fn print_vector_len(vector: &Vec<i32>) {
    println!("Vector length: {}", vector.len());
}

pub fn print_memory_access(count: i32) {
    println!("Memory accesses count: {}", count);
}

pub fn print_comparisons(count: i32) {
    println!("Comparisons count: {}", count);
}

pub fn print_elapsed_time(time: i32) {
    println!("Time in milliseconds: {}", time);
}

pub fn print_err(err: String) {
    println!("Error: {}", err);
}
