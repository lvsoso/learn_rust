fn avg(a: u32, b: u32) -> u32 {
    (a & b) + ((a ^ b) >> 1)
}

fn overflowing() {
    let a: u32 = 4294967295;
    let b: u32 = 1;

    let (r, is_overflow) = a.overflowing_add(b);
    println!("r={} is_overflow={}", r, &is_overflow);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_overflowing() {
        overflowing();
    }

    #[test]
    fn test_avg() {
        assert_eq!(avg(4294967295, 4294967295), 4294967295);
        assert_eq!(avg(0, 0), 0);
        assert_eq!(avg(10, 20), 15);
        assert_eq!(avg(4294967295, 1), 2147483648);
        println!("passed");
    }
}
