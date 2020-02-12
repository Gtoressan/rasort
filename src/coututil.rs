pub fn print_vector(vector: &Vec<i32>, columns: u128) {
    let mut current_column = 0;
    let mut printing_flag = false;

    for i in 0..vector.len() {
        if current_column < columns {
            print!("{}\t", vector[i]);
            current_column += 1;
            printing_flag = true;
        }

        if printing_flag && (current_column == columns || i + 1 >= vector.len()) {
            current_column = 0;
            println!();
        }
    }
}

pub fn print_vector_len(vector: &Vec<i32>) {
    println!("Vector length: {}", vector.len());
}

pub fn print_memory_access(count: u128) {
    println!("Memory accesses count: {}", count);
}

pub fn print_comparisons(count: u128) {
    println!("Comparisons count: {}", count);
}

pub fn print_elapsed_time(time: u128) {
    println!("Time in milliseconds: {}", time);
}

pub fn print_err(err: String) {
    println!("Error: {}", err);
}
