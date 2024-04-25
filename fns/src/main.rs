//normal struct
struct Person{
    name:String,
    age:i32
}

//tuple Struct
struct Color(i32,i32,i32);

fn main() {


    let red:Color=Color(255,0,0);

    let Rushikesh:Person=Person{name:"Rushikesh".to_string(),age:25};
    println!("{}",Rushikesh.name);
    println!("{}",Rushikesh.age);



    // print_numbers(5)
    // refs();
}

fn refs(){
    let mut x=4;
    let  y=&mut x;
    *y+=1;
    println!("{}",x)
}


fn print_numbers(numbers:i32)
{
    for i in 0..numbers{
        println!("{}",i)
    }
}