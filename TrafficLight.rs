//约定接口，返回一个时间方法，返回类型i32，单位秒
trait ITime{
    fn time(&self)->i32;
}

//定义交通信号灯，红黄绿三种状态
enum  TrafficLight{
    Red,
    Yellow,
    Green,
}

//为TrafficLight实现ITime接口，输出灯各种状态实间
impl ITime for TrafficLight{
    //红黄绿三种状态持续时间，单位秒
    fn time(&self)->i32{
        match self{
            //时间只是为了测试
            TrafficLight::Yellow =>3,
            TrafficLight::Red =>30,
            _ =>60
        }
    }
}

//重构测试程序
fn test(tl:TrafficLight){
    
    println!("Traffic {} light Seconds {}",tl.to_string(),tl.time());
}

//将交通信号灯枚举转换为字符串
impl TrafficLight {
    fn to_string(&self) -> String{ 
        match self{
            TrafficLight::Yellow => String::from("Yellow"),
            TrafficLight::Red => String::from("Red"),
            TrafficLight::Green => String::from("Green"),
            _ =>String::from("Undefined"),          
        }
    }
}


fn main() {
    //调用重构测试程序
    test(TrafficLight::Green);
    test(TrafficLight::Red);
    test(TrafficLight::Yellow);

    /*
   //测试三种状态交通灯持续时间
  let red_light = TrafficLight::Red;
  let yellow_light = TrafficLight::Yellow;
  let geen_light = TrafficLight::Yellow;
  println!("Traffic light Seconds:Red:{},Yellow:{},Green:{}",
    red_light.time(),yellow_light.time(),geen_light.time());
    */
}
