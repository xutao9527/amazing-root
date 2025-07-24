// use std::fs::{read_to_string, write};
// use crate::crypto::decode::decode;
// 
// pub fn decode_file(input_path: &str, output_path: &str){
//     let content = read_to_string(input_path).expect("Failed to read input file");
//     let bytes = decode(&content);
//     write(output_path, &bytes).expect("Failed to write output file");
// }