//Typecheck Code
// use std::any::type_name_of_val;

// fn main() {
//     let  a=String::from("rushikesh");
//     println!("{}",type_name_of_val(&a));
// }


fn main(){

    let s:String=String::from("hi therre my name is Rushikesh");
    println!("{}",s.len());

    for ch in s.split(" "){
        println!("{}",ch);
    }
}