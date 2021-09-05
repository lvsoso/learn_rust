mod mod1 {
    pub mod mod2 {
        pub const MESSAGE: &str = "HelloWorld!";
    }
}

#[cfg(test)]
mod tests {
    use super::mod1::mod2::MESSAGE;

    #[test]
    fn test_mod() {
        println!("{}", MESSAGE);
    }
}
