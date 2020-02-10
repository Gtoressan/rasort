pub struct State {
    pub print_unsorted_vector: bool,
    pub print_sorted_vector: bool,
    pub print_comparisons: bool,
    pub print_memory_accesses: bool,
    pub print_elapsed_time: bool,
    pub print_vector_length: bool,
}

pub struct StateValue {
    pub unsorted_vector: Vec<i32>,
    pub sorted_vector: Vec<i32>,
    pub comparisons: u128,
    pub memory_accesses: u128,

    // Time counts in milliseconds.
    pub elapsed_time: u128,
}
