// use std::fs::{read, write};
// use crate::crypto::encode::encode;
//
// pub fn encode_file(input_path: &str, output_path: &str){
//     println!("Encoding file: {}", input_path);
//     let data = read(input_path).expect("Failed to read input file");
//     let encoded = encode(&data);
//     write(output_path, encoded).expect("Failed to write output file");
// }