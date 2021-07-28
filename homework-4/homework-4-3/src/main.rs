
fn main() {
    let r1 = Rectangle {
        length: 5.0,
        height: 10.0,
    };
    //get rectangle's area
    println!("r1 area = {}", r1.area());
    println!("area of r1 = {}", area(&r1));

    let t1 = Triangle {
        length: 15.0,
        height: 22.0,
    };
    // get triangle's area
    println!("t1 area = {}", t1.area());
    println!("area of t1 = {}", area(&t1));
}


struct Rectangle {
    length: f64,
    height: f64,
}
struct Triangle {
    length: f64,
    height: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}
impl HasArea for Triangle {
    fn area(&self) -> f64 {
        self.length * self.height / 2.0
    }
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}
