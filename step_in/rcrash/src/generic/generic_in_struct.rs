
#[derive(Debug)]
struct Point<T, U> {
    x : T,
    y: T,
    z: U,
}


#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn test_point() {
        let integer = Point{x:5, y:10, z:15.0};
        let float  = Point{x: 1.0, y : 4.0, z:8};

        println!("{:?}", integer);
        println!("{:?}", float);
    }
}
