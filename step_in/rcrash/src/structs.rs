struct Pair(i32, f32);

struct Person {
    name: String,
    age: u8,
}

struct Uint;

fn example() {
    let pair = Pair(10, 4.2);
    let person = Person {
        name: String::from("jack"),
        age: 21,
    };
    let uint = Uint;

    println!("{}", pair.0);
    println!("{}", person.name);
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn test_example() {
        example();
    }
}
