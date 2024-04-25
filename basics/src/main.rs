fn main() {
    let a=1;   //Immutable
    let mut b=2;   //Mutable
    let c:i32=3;   //Immutable
    let d:bool=true;   //Immutable
    let mut b=2;
    let c:i32=3;
    let d:bool=true;

    if true{
        println!("Hello World from if block");
    }  
    else{
        println!("Hello World from else block");
    }

    let mut i=0;
    loop{
        i+=1;
        if i==6{
            continue;
        }
        println!("{}",i);
        if i==10{
            break;
        }
    }

}
