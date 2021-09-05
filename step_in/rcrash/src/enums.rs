enum Planet {
    Mars,
    Earth,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum IpAddr {
    IPv4(u8, u8, u8, u8),
    IPv6(
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
    ),
}

fn example() {
    let localhost: IpAddr = IpAddr::IPv4(127, 0, 0, 1);
    match localhost {
        IpAddr::IPv4(a, b, c, d) => {
            println!("{} {} {} {}", a, b, c, d);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn test_example() {
        example();
    }
}
