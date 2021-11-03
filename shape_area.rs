use std::fmt::Debug;
//#[derive(Debug)]

//统一打印图形面积，约束类型是IArea+Debug格式输出图形面积
fn print_area<T: IArea+Debug>(shape: T) {
    println!("图形:{:?}, 面积:{}",shape, shape.area());
}

//接口定义，求面积
trait IArea{
    //定义接口方法而不实现
    fn area(&self)->f64;
}

//定义圆形
#[derive(Debug)]
struct Circle {
    x:f64,
    y:f64,
    radius:f64,
}

//实现IArea接口
impl IArea for Circle{
    fn area(&self) ->f64 {
        std::f64::consts::PI* (self.radius *self.radius)
    }
}

//定义三角形
#[derive(Debug)]
struct Triangle{
    bottom_margin:f64,	//底边
    height:f64,			//高
}

//实现IArea接口
impl IArea for Triangle
{
    fn area(&self)->f64{
        self.bottom_margin * self.height / 2.0f64
    }
}

//正方形
#[derive(Debug)]
struct Square{
   length:f64		//边长
}

//实现IArea接口
impl IArea for Square{
    fn area(&self)->f64{
        self.length * self.length
    }
}

fn main() {
    let c =Circle {
        x:0.0f64,
        y:0.0f64,
        radius:5.0f64,
    };
    //println!("circle c has an area of {}", c.area());
    print_area(c);
    
    let triangle =Triangle {
        bottom_margin:10.0f64,
        height:5.2f64,
    };
    //println!("triangle c has an area of {}", triangle.area());
    print_area(triangle);
    
    let square = Square{
        length:10.0f64
    };
    //println!("square c has an area of {}", square.area());
    print_area(square);
    
}
