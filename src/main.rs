use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    let token : String= read_token(file_path);

    println!("{token}");

    parse_token(&token);



}

fn read_token(file_path : &String) -> String {

    let contents: String = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");


    return contents;

}

fn parse_token(token : &String) -> String {
    let parsed_token= String::from(token);

    return parsed_token;
}