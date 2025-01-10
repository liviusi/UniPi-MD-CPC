use std::fs::File;
use std::io::{BufRead, BufReader};

use handson3::{design_a_course, holiday_planning};

fn main() {
    test_one();
    test_two();
}

fn test_one() {
    let folder_path = "./tests-1/";
    for file_index in 0..5 {
        let input_path = format!("{}input{}.txt", folder_path, file_index);
        let outpat_path = format!("{}output{}.txt", folder_path, file_index);
        if let Ok(input_file) = File::open(&input_path) {
            if let Ok(output_file) = File::open(&outpat_path) {
                let input_reader = BufReader::new(input_file);
                let mut output_lines = BufReader::new(output_file).lines();

                let mut days_no: usize = 0;
                let mut len: usize = 0;
                let mut cities = vec![];

                for (line_no, line_wrapper) in input_reader.lines().enumerate() {
                    let line = line_wrapper.unwrap();
                    let content: Vec<u32> = line
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    if line_no == 0 {
                        len = content[0] as usize;
                        days_no = content[1] as usize;
                    } else if !content.is_empty() {
                        cities.push(content);
                    } else if line_no > len + 1 {
                        assert_eq!(
                            holiday_planning(cities.clone(), days_no),
                            output_lines
                                .next()
                                .unwrap()
                                .unwrap()
                                .parse::<u32>()
                                .unwrap()
                        )
                    }
                }
            } else {
                panic!(
                    "Fatal error occurred attempting to open file {}",
                    outpat_path
                );
            }
        } else {
            panic!(
                "Fatal error occurred attempting to open file {}",
                input_path
            );
        }

        println!("[Problem 1] Test {} completed successfully", file_index);
    }
    println!();
}

fn test_two() {
    let folder_path = "./tests-2/";
    for file_index in 0..11 {
        let input_path = format!("{}input{}.txt", folder_path, file_index);
        let outpat_path = format!("{}output{}.txt", folder_path, file_index);
        if let Ok(input_file) = File::open(&input_path) {
            if let Ok(output_file) = File::open(&outpat_path) {
                let input_reader = BufReader::new(input_file);
                let mut output_lines = BufReader::new(output_file).lines();

                let mut len: usize = 0;
                let mut topics = vec![];

                for (line_no, line_wrapper) in input_reader.lines().enumerate() {
                    let line = line_wrapper.unwrap();
                    let content: Vec<u32> = line
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    if line_no == 0 {
                        len = content[0] as usize;
                    } else if !content.is_empty() {
                        topics.push((content[0] as i32, content[1] as i32));
                    } else if line_no > len + 1 {
                        assert_eq!(
                            design_a_course(topics.clone()),
                            output_lines
                                .next()
                                .unwrap()
                                .unwrap()
                                .parse::<usize>()
                                .unwrap()
                        )
                    }
                }
            } else {
                panic!(
                    "Fatal error occurred attempting to open file {}",
                    outpat_path
                );
            }
        } else {
            panic!(
                "Fatal error occurred attempting to open file {}",
                input_path
            );
        }

        println!("[Problem 2] Test {} completed successfully", file_index);
    }
    println!();
}
