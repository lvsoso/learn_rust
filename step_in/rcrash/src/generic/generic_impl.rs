
#[derive(Debug)]
struct Point<T> {
    x : T,
    y: T,
}

impl <T: std::cmp::PartialOrd +Clone> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn largest(&self) -> T {
        if self.x > self.y {
            self.x.clone()
        } else {
            self.y.clone()
        }
    }
    
}


// 只为某种特殊的泛型类型提供该方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn test_point() {
    let p = Point{x:5, y : 10};
    println!("p.x = {}", p.x());
    }


    #[test]
    fn test_largest() {
        let point = Point{x:10, y:20};
        println!("{:?}", point.largest());
        // println!("{}", p.distance_from_origin());
    }

    #[test]
    fn test_float_point() {
        let p = Point{x: 1.0, y:2.0};
        println!("{}", p.distance_from_origin());
    }
}
