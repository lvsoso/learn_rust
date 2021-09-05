fn example() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &arr[0..3];
    println!("slice[0] = {}, len={}", slice[0], slice.len());
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn test_example() {
        example();
    }
}
