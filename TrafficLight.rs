enum  TrafficLight{
    Red,
    Yellow,
    Green,
}

impl TrafficLight{
    fn time(&self)->i32{
        match self{
            TrafficLight::Yellow =>3,
            _ =>60
        }
    }
}

fn main() {
  let red_light = TrafficLight::Red;
  let yellow_light = TrafficLight::Yellow;
  let geen_light = TrafficLight::Green;
  println!("Traffic light Seconds:Red:{},Yellow:{},Green:{}",
    red_light.time(),yellow_light.time(),geen_light.time());
}
