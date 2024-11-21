use std::fs::File;
use std::io::{BufRead, BufReader};

use handson2::{SegmentTreeFirstExercise, SegmentTreeSecondExercise};

fn main() {
    test_one();
    test_two();
}

fn test_one() {
    let folder_path = "./tests-1/";
    for file_index in 0..11 {
        // there are 11 files numbered 0 to 10
        let input_path = format!("{}input{}.txt", folder_path, file_index);
        let outpat_path = format!("{}output{}.txt", folder_path, file_index);
        if let Ok(input_file) = File::open(&input_path) {
            if let Ok(output_file) = File::open(&outpat_path) {
                let input_reader = BufReader::new(input_file);
                let mut output_lines = BufReader::new(output_file).lines();

                let mut segment_tree = SegmentTreeFirstExercise::from_vector(vec![-1]);
                let mut len = 0;

                for (line_no, line_wrapper) in input_reader.lines().enumerate() {
                    let line = line_wrapper.unwrap();
                    let content: Vec<i32> = line
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    if line_no == 0 {
                        len = content[0] as usize;
                        segment_tree = SegmentTreeFirstExercise::from_vector(content);
                    } else if !content.is_empty() {
                        if content[0] == 0 {
                            segment_tree.update(
                                content[1] as usize,
                                content[2] as usize,
                                content[3],
                            );
                        } else if line_no > len + 1 {
                            assert_eq!(
                                segment_tree.max(content[1] as usize, content[2] as usize),
                                output_lines
                                    .nth(line_no - len + 1)
                                    .unwrap()
                                    .unwrap()
                                    .parse::<i32>()
                                    .unwrap()
                            )
                        }
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
    for file_index in 0..8 {
        // there are 8 files numbered 0 to 7
        let input_path = format!("{}input{}.txt", folder_path, file_index);
        let outpat_path = format!("{}output{}.txt", folder_path, file_index);
        if let Ok(input_file) = File::open(&input_path) {
            if let Ok(output_file) = File::open(&outpat_path) {
                let input_reader = BufReader::new(input_file);
                let mut output_lines = BufReader::new(output_file).lines();

                let mut segments = Vec::new();
                let mut queries = Vec::new();
                let mut n = 0;

                for (line_no, line_wrapper) in input_reader.lines().enumerate() {
                    let line = line_wrapper.unwrap();
                    let content: Vec<usize> = line
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    if line_no == 0 {
                        n = content[0];
                    } else if !content.is_empty() {
                        if line_no < n + 1 {
                            segments.push((content[0], content[1]));
                        } else {
                            queries.push((content[0], content[1], content[2]));
                        }
                    }
                }
                let mut segment_tree = SegmentTreeSecondExercise::from_segment_vector(segments);
                for query in queries {
                    assert_eq!(
                        output_lines
                            .next()
                            .unwrap()
                            .unwrap()
                            .parse::<i32>()
                            .unwrap(),
                        segment_tree.is_there(query.0, query.1, query.2 as u32)
                    );
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
}
