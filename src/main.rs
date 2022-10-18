mod base64map;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input_in_binary = String::new();
    let mut output_in_base64 = String::new();

    for byte in args[1].as_bytes() {
        input_in_binary += &format!("{:08b}", byte);
    }
    for chunk in input_in_binary.chars().collect::<Vec<char>>().chunks(6) {
        let chunk_string = format!("{:0<6}", chunk.iter().collect::<String>());
        output_in_base64 += base64map::get_base64_char(&chunk_string);
    }

    if output_in_base64.len() % 4 != 0 {
        output_in_base64 += &"=".repeat(4 - output_in_base64.len() % 4);
    }

    println!("{}", output_in_base64);
}
