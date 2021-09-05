// Default 字段自动使用默认值
#[derive(Debug, PartialEq, Default)]
struct Point{
    x : i32,
    y: i32,
}



#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn test_derive() {
        let point = Point{x: 10, y : 20};
        let point2 = Point{x: 10, y : 20};
        println!("{:?}", point);

        println!("{:?}", point == point2);
    }

    #[test]
    fn test_defualt() {
        let point = Point::default();
        println!("{:?}", point);
    }
}


