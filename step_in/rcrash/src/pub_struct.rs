mod mod1 {
    pub struct Person {
        pub name: String,
        nickname: String,
    }

    impl Person {
        pub fn new(name: &str, nickname: &str) -> Self {
            Person {
                name: String::from(name),
                nickname: String::from(nickname),
            }
        }

        pub fn set_nickname(&mut self, nickname: &str) {
            self.nickname = nickname.to_string();
        }

        pub fn say_nick_name(&self) {
            println!("{}", self.nickname);
        }
    }
}

fn pub_struct() {
    let mut p = mod1::Person::new("jack", "baby");
    println!("{}", p.name);
    //  不能直接访问 nickname；
    // println!("{}",  p.nickname);
    p.say_nick_name();
    p.set_nickname("jaja");
    p.say_nick_name();
}

#[cfg(test)]
mod tests {

    use super::pub_struct;

    #[test]
    fn test_pub_struct() {
        pub_struct()
    }
}
