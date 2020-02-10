mod cinutil;
mod coututil;
mod args;

#[path = "./algos/bubblesort.rs"]
mod bubblesort;
#[path = "./algos/mergesort.rs"]
mod mergesort;

#[macro_use]
extern crate clap;

use std::process;
use std::time;


fn main() {
    let yaml = load_yaml!("../configs/cli.yml");
    let matches = clap::App::from_yaml(yaml)
        .get_matches();

    // Requested arguments.
    let file = matches.value_of("file").unwrap();
    let algo = matches.value_of("algorithm-name").unwrap();

	let config = args::Config {
        // Optional flags.
        UNSORTED_VECTOR: matches.is_present("unsorted-vector"),
        UNSORTED_VECTOR_COLUMNS: match matches.value_of("unsorted-vector") {
            Some(value) => {
                match value.parse() {
                    Ok(columns) => columns,
                    Err(_) => {
                        coututil::print_err(format!("invalid columns count"));
                        process::exit(1);
                    }
                }
            },
            None => 0
        },
        SORTED_VECTOR: matches.is_present("sorted-vector"),
        SORTED_VECTOR_COLUMNS: match matches.value_of("sorted-vector") {
            Some(value) => {
                match value.parse() {
                    Ok(columns) => columns,
                    Err(_) => {
                        coututil::print_err(format!("invalid columns count"));
                        process::exit(1);
                    }
                }
            },
            None => 0
        },
        COMPARISONS: matches.is_present("comparisons"),
        MEMORY_ACCESSES: matches.is_present("memory-accesses"),
        TIME: matches.is_present("time"),
        VECTOR_LEN: matches.is_present("vector-len"),
    };

    let mut vector = match cinutil::get_vec_from(&file) {
        Ok(vector) => {
			if config.UNSORTED_VECTOR {
				coututil::print_vector(&vector, config.UNSORTED_VECTOR_COLUMNS);
			}

            vector
        },
        Err(err) => {
            coututil::print_err(err);
            process::exit(1);
        }
    };

    let start: time::Instant;
    let end: time::Instant;

    match algo {
        "bubble" => {
            start = time::Instant::now();
            bubblesort::bubblesort(&mut vector);
            end = time::Instant::now();
        },
        "merge" => {
            start = time::Instant::now();
            vector = mergesort::mergesort(vector);
            end = time::Instant::now();
        },
        _ => {
            coututil::print_err(format!("provided algorithm name not recognized"));
            process::exit(1)
        }
    }

	if config.TIME {
		coututil::print_elapsed_time(end.duration_since(start).as_millis());
	}
	if config.VECTOR_LEN {
		coututil::print_vector_len(&vector);
	}
	if config.SORTED_VECTOR {
		coututil::print_vector(&vector, config.SORTED_VECTOR_COLUMNS);
	}
}
