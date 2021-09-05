use std::thread;

fn closure1() {
    let myclosure = |n: u32| -> u32 { n * 3 };
    println!("{}", myclosure(1))
}

fn closure2() {
    let hello_message = "Hello World!";
    thread::spawn(move || println!("{}", hello_message)).join();
}

// =============================================================================

type Method = fn(u32, u32) -> u32;

// function as a param
fn calc(method: Method, a: u32, b: u32) -> u32 {
    method(a, b)
}

// function as return value
fn calc_match(method: &str) -> Method {
    match method {
        "add" => add,
        "sub" => sub,
        _ => unimplemented!(),
    }
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn sub(a: u32, b: u32) -> u32 {
    a - b
}

fn advance() {
    println!("add {}", calc(add, 10, 20));
    println!("sub {}", calc(sub, 20, 10));

    println!("add {}", calc_match("add")(10, 20));
    println!("sub {}", calc_match("sub")(20, 10));
}

// ================================================================

// divergency function
// It is diffrent with function which return empty value.
// It never return.

fn foo() -> ! {
    panic!("This call never returns.");
}

fn some_fn() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure1() {
        closure1();
    }

    #[test]
    fn test_closure2() {
        closure2();
    }

    #[test]
    fn test_advance() {
        advance();
    }

    #[test]
    fn test_some_fn() {
        some_fn();
    }

    #[test]
    fn test_foo() {
        let a = if true { 10 } else { foo() };
        println!("{}", a);
    }
}
