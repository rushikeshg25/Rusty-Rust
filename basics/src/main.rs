fn main() {
    // let a=1;   //Immutable
    // let mut b=2;   //Mutable
    // let c:i32=3;   //Immutable
    // let d:bool=true;   //Immutable
    // let mut b=2;
    // let c:i32=3;
    // let d:bool=true;

    // //If else Syntax 

    // if true{
    //     println!("Hello World from if block");
    // }  
    // else{
    //     println!("Hello World from else block");
    // }

    // //Loop Syntax using Loop keyword

    // let mut i=0;
    // loop{
    //     i+=1;
    //     if i==6{
    //         continue;
    //     }
    //     println!("{}",i);
    //     if i==10{
    //         break;
    //     }
    // }

    // // Loop Syntax using While keyword

    // i=0;
    // while i<6{
    //     i+=1;
    //     println!("{}",i);
    //     if i==10{
    //         break;
    //     }
    // }

    // //Loop Syntax using For loop

    // for i in 0..6{
    //     println!("{}",i);
    // }

    //For loop using vector

    let animals= vec!["cat","dog","bird"];

    //use this to aviod not be vector accessible instead of below syntax
    for animal in animals.iter(){
        println!("{}",animal);
    }

    for (index,animal) in animals.iter().enumerate(){
        println!("The Index is {} and the animal is {}",index,animal);
    }

    for animal in animals{
        println!("{}",animal);
    }

    // For loop using array

    let numbers= [1,2,3,4,5];
    for number in numbers{
        println!("{}",number);
    }

}
