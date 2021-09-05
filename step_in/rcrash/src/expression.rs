fn if_expression() {
    let mut s = 0;
    let mut n = 10;
    let a = loop {
        if n < 0 {
            break s;
        }
        s += n;
        n -= 1;
    };

    println!("{:?}", a);
}

fn loop_expression() {
    let mut sum = 0;
    let mut n = 0;
    loop {
        sum += n;
        n += 1;
        if n > 100 {
            break;
        }
    }
    println!("{:?}", sum);
}

fn break_expression() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}

fn while_expression() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}

fn for_range_expression() {
    for i in 0..5 {
        println!("{}", i);
    }

    for i in 0..=5 {
        println!("{}", i);
    }
}

fn for_range_iter_expression() {
    let mut myarray = [1, 2, 3];
    for i in myarray.iter() {
        println!("{}", i);
    }

    for i in myarray.iter_mut() {
        *i *= 2;
    }

    println!("---------------");

    for i in myarray.iter() {
        println!("{}", i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_expression() {
        if_expression();
    }

    #[test]
    fn test_break_expression() {
        break_expression();
    }

    #[test]
    fn test_loop_expression() {
        loop_expression();
    }

    #[test]
    fn test_while_expression() {
        while_expression();
    }

    #[test]
    fn test_for_range_expression() {
        for_range_expression();
    }

    #[test]
    fn test_for_range_iter_expression() {
        for_range_iter_expression();
    }
}
