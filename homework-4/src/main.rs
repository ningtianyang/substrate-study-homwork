
fn main() {
    println!("==== Question 1 ====");

    // Question 1
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("Red time = {}", red.time());
    println!("Yellow time = {}", yellow.time());
    println!("Green time = {}", green.time());

    println!("==== Question 2 ====");
    // Question 2
    let normal_vec = vec![1111 2333, 4242, 24112, 5333, 10, 2245, 124];
    let sum_of_normal_vec = sum_of_vec(&normal_vec);
    let sum_of_normal_vec_value = match sum_of_normal_vec {
        Some(value) => value,
        None => 0,
    };
    println!("sum_of_normal_vec_value = {}", sum_of_normal_vec_value);

    let empty_vec = vec![];
    let sum_of_empty_vec = sum_of_vec(&empty_vec);
    let sum_of_empty_vec_value = match sum_of_empty_vec {
        Some(value) => value,
        None => 0,
    };
    println!("sum_of_empty_vec_value = {}", sum_of_empty_vec_value);

    // Question 3
    println!("==== Question 3 ====");
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

// Question 1
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait TrafficLightTime {
    fn time(&self) -> u8;
}

impl TrafficLightTime for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 20,
            TrafficLight::Green => 60,
        }
    }
}

// Question 2
fn sum_of_vec(some_vec: &[u32]) -> Option<u32> {
    let mut result: u32 = 0;
    for i in some_vec.iter() {
        match result.checked_add(*i) {
            Some(added_result) => result = added_result,
            None => return None,
        }
    }
    Some(result)
}

// Question 3
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
