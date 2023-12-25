fn give_string(s: String) -> String {
    s
    
}
fn first_word(s: &mut String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item)in bytes.iter().enumerate() { 
        if item == b' ' {
            return &s[..i]; 
        }
    }
    &s[..]
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _) = tup;
    let val = if tup.0 == 500 {
        1
    }
    else {
        2
    };
    println!("{}", val);
    let a = [10, 20, 30, 40, 50];
    for element in a.iter().rev() {
        println!("the value is: {}", element);
    }
    let s1 = String::from("hello world");
    let mut s1 = give_string(s1);
    let s2: &str = first_word(&mut s1);
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:#?}", rect1);
    println!("{}", s2);
    let val = rect1.area();
    println!("{}", val);
}

