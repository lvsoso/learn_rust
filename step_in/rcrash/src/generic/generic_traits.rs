
struct Point<T> {
    x : T,
    y: T,
}


// 使用Traits定义共同的行为
impl  <T: std::fmt::Display>  std::fmt::Display for Point<T> {
    fn fmt(&self,  f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 使用Traits作为参数类型
// fn show <T:  std::fmt::Display>(a: T){
//     println!("show: {}", a);
// }

// impl
fn show(a: impl std::fmt::Display){
    println!("show: {}", a);
}

#[cfg(test)]
mod tests {
    use super::Point;
    use super::show;

    #[test]
    fn test_dispaly() {
        let point = Point{x: 10, y : 20};
        println!("{}", point);
    }

    #[test]
    fn test_show() {
        let integer = Point{x:5, y:10};
        show(integer);
    }
}


