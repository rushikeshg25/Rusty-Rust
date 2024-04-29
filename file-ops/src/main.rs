//Packages for reading File
use std::fs::File;
use std::io::prelude::*;  

fn main() {
    //Getting File path and reading the content
    let mut file=File::open("hello.txt").expect("file not found");
    let mut contents=String::new();
    file.read_to_string(&mut contents).expect("error in reading");
    //Printing COntent
    println!("{}",contents)
    

}
