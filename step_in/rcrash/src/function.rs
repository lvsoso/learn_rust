// F(0) = 0
// F(1) = 1
// F(n) = F(n-1) + F(n-2)

fn fibonacci(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Point {
        Point { x, y }
    }

    fn get_x(&self) -> u64 {
        return self.x;
    }

    fn get_y(&self) -> u64 {
        return self.y;
    }

    fn set_x(&mut self, x: u64) {
        self.x = x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let y = fibonacci(8);
        println!("{}", y);
    }

    #[test]
    fn test_Point() {
        let mut p = Point::new(10, 20);
        println!("{:?}", p);
        println!("{:?}", p.get_x());
        p.set_x(5);
        println!("{:?}", p.get_x());
    }
}
