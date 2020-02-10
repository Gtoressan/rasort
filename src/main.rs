#[path = "./algos/bubblesort.rs"]
mod bubblesort;
#[path = "./algos/mergesort.rs"]
mod mergesort;

mod args;
mod cinutil;
mod coututil;

#[macro_use]
extern crate clap;

use std::process;
use std::time;


fn main() {
    let yaml = load_yaml!("../configs/cli.yml");
    let matches = clap::App::from_yaml(yaml)
        .get_matches();

    // Required arguments.
    let file = matches.value_of("file").unwrap();
    let algo = matches.value_of("algorithm-name").unwrap();

    // Optional arguments.
    let state = args::State {
        print_comparisons: matches.is_present("comparisons"),
        print_elapsed_time: matches.is_present("time"),
        print_memory_accesses: matches.is_present("memory-accesses"),
        print_sorted_vector: matches.is_present("sorted-vector"),
        print_unsorted_vector: matches.is_present("unsorted-vector"),
        print_vector_length: matches.is_present("vector-len"),
        sorted_vector_columns: match matches.value_of("sorted-vector") {
            Some(value) => {
                match value.parse() {
                    Ok(number) => number,
                    Err(_) => {
                        coututil::print_err(format!("invalid input value"));
                        process::exit(1);
                    }
                }
            },
            None => 0
        },
        unsorted_vector_columns: match matches.value_of("unsorted-vector") {
            Some(value) => {
                match value.parse() {
                    Ok(number) => number,
                    Err(_) => {
                        coututil::print_err(format!("invalid input value"));
                        process::exit(1);
                    }
                }
            },
            None => 0
        },
    };

    let mut state_value = args::StateValue {
        comparisons: 0,
        memory_accesses: 0,
        elapsed_time: 0,
        vector: match cinutil::get_vec_from(file) {
            Ok(vector) => {
                if state.print_unsorted_vector {
                    coututil::print_vector(&vector, state.unsorted_vector_columns);
                }

                vector
            }
            Err(err) => {
                coututil::print_err(format!("{}", err));
                process::exit(1);
            }
        }
    };

    match algo {
        "bubble" => {
            let start = time::Instant::now();
            bubblesort::bubblesort(&mut state_value.vector);
            state_value.elapsed_time = start.elapsed()
                .as_millis();
        },
        "merge" => {
            let start = time::Instant::now();
            state_value.vector = mergesort::mergesort(state_value.vector);
            state_value.elapsed_time = start.elapsed()
                .as_millis();
        },
        _ => {
            coututil::print_err(format!("provided algorithm name not recognized"));
            process::exit(1)
        }
    }

    if state.print_comparisons {
        coututil::print_comparisons(state_value.comparisons);
    }
    if state.print_elapsed_time {
        coututil::print_elapsed_time(state_value.elapsed_time);
    }
    if state.print_memory_accesses {
        coututil::print_memory_access(state_value.memory_accesses);
    }
    if state.print_vector_length {
        coututil::print_vector_len(&state_value.vector);
    }
    if state.print_sorted_vector {
        coututil::print_vector(&state_value.vector, state.sorted_vector_columns);
    }
}
