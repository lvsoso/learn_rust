enum Alphabet {
    A,
    B,
}

fn if_let() {
    let letter = Alphabet::A;
    match letter {
        Alphabet::A => {
            println!("It is A");
        }
        _ => {}
    }

    if let Alphabet::A = letter {
        println!("It is A");
    }
}

enum Symbol {
    Char(char),
    Number,
}

fn if_let_param() {
    let letter = Symbol::Char('A');

    if let Symbol::Char(x) = letter {
        println!("{:?}", x);
    }
}

fn while_let() {
    let mut letter = Alphabet::A;

    while let Alphabet::A = letter {
        println!("It is A");
        letter = Alphabet::B;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_let() {
        if_let();
    }

    #[test]
    fn test_if_let_param() {
        if_let_param();
    }

    #[test]
    fn test_while_let() {
        while_let();
    }
}
