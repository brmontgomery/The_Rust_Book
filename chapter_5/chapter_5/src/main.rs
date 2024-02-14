//outer attribute
#[derive(Debug)]
//declare struct just like C++
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    //create instance of struct just like C++. You can make a whole struct mutable or not. You cannot do partial mutable instances of structs
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    //:? gives debug output (small structs, all one line)
    //:#? gives debug output that is vertically separated (large structs, multiple lines)
    //can also use dbg!
    //in debug you can pass by reference to it to maintain ownership of the struct.
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}


//function uses a passed by reference struct to carry out its function
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//or use a struct method (declared byputting impl[STRUCT] in front of the function)
//this is then called like a class function in C++ impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}