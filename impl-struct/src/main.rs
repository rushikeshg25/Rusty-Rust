struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle{
    fn desc(&self){
        println!("{}*{}",self.width,self.height)
    }
}

fn main() {
    let my_rectangle=Rectangle{width:10,height:30};
    my_rectangle.desc();
}
