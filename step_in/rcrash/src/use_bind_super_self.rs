// bind  std::fs -> fs
use std::fs;
// bind std::fs -> stdfs
use std::fs as stdfs;

fn bind() {
    let data = fs::read("src/main.rs").unwrap();
    println!("{}", String::from_utf8(data).unwrap());
}

fn function() {
    println!("function");
}

mod mod1 {
    pub fn function() {
        super::function();
    }

    pub mod mod2 {
        fn function() {
            println!("mod1::mod2::function");
        }

        pub fn call() {
            self::function();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::mod1;

    #[test]
    fn test_super() {
        mod1::function();

        // mod1::mod2::function();
    }

    #[test]
    fn test_self() {
        mod1::mod2::call();
    }
}
