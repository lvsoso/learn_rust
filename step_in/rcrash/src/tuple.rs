fn tuple() {
    let a: i32 = 10;
    let b: char = 'A';

    let mytuple: (i32, char) = (a, b);

    println!(".0={:?}", mytuple.0);
    println!(".1={:?}", mytuple.1);

    let (c, d) = mytuple;
    println!("c={} d={}", c, d);
}

#[cfg(test)]
mod tests {
    use super::tuple;

    #[test]
    fn test_tuple() {
        tuple();
    }
}
