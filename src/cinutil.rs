use std::fs;

pub fn get_vec_from(file: &str) -> Result<Vec<i32>, String> {
    match fs::read_to_string(file) {
        Ok(text) => {
            let mut vector = Vec::<i32>::new();

            for i in text.split_whitespace() {
                match i.parse() {
                    Ok(number) => {
                        vector.push(number);
                    },
                    Err(_) => {
                        return Err(format!("failed when parsing file"));
                    }
                }
            }
            
            Ok(vector)
        },
        Err(err) => Err(format!("{}", err))
    }
}
