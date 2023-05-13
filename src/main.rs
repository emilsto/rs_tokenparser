use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    let contents = read_token(file_path).unwrap();
    println!("{}", contents);



}

fn read_token(file_path : &String) -> Result<String, std::io::Error> {

    let contents = fs::read_to_string(file_path)?;
    Ok(contents)

}

fn parse_token(token : &String) -> String {
    let parsed_token= String::from(token);

    return parsed_token;
}