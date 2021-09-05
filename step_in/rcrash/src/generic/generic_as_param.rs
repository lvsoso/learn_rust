fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::largest;

    #[test]
    fn test_largest() {
        println!("{}", largest::<u32>(1, 2));
        println!("{}", largest::<f32>(1.0, 2.0));
    }
}
