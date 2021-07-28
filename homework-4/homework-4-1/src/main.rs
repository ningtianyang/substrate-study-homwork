
fn main() {

    let red = TrafficLight::Red;
    println!("Red time = {}", red.time());


    let yellow = TrafficLight::Yellow;
    println!("Yellow time = {}", yellow.time());

    let green = TrafficLight::Green;
    println!("Green time = {}", green.time());

}


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
