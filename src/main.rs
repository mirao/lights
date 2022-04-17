use crate::light::houselight::HouseLight;
use crate::light::trafficlight::TrafficLight;
use crate::light::Light;

mod light;

fn main() {
    let mut traffic_light = TrafficLight::new();
    let house_light = HouseLight::new();

    print_state(&traffic_light);
    traffic_light.turn_green();
    print_state(&traffic_light);
    traffic_light.turn_red();
    print_state(&traffic_light);
    traffic_light.turn_yellow();
    print_state(&traffic_light);
    // Display mode
    println!("{}", traffic_light);
    // Debug mode
    println!("{:?}", traffic_light);

    print_state(&house_light);
    // Display mode
    println!("{}", house_light);
    // Debug mode
    println!("{:?}", house_light);
}

fn print_state(light: &impl Light) {
    println!("{}'s state is : {:?}", light.get_name(), light.get_state());
}
