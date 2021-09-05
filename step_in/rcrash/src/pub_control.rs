// pub 成员对模块可见
// pub(self) 成员对模块内的子模块可见
// pub(crate) 成员对整个crate可见
// 不使用 pub 声明，成员默认的可见性是私有的

mod mod1 {
    pub const MESSAGE: &str = "Hello World!";

    // 私有的
    // pub(self) const  NUMBER : u32 =  3344;
    const NUMBER: u32 = 3344;

    pub(self) fn mod1_pub_self_fn() {
        println!("{}", NUMBER);
    }

    // 整个crate都可见
    pub(crate) enum CrateEnum {
        Item = 4,
    }

    pub mod mod2 {
        use crate::pub_control::mod1::NUMBER;

        // 它的可见性受父级影响
        pub const MESSAGE: &str = "Hello World!";

        pub fn mod2_fn() {
            super::mod1_pub_self_fn();
        }

        pub fn print_number() {
            println!("{}", super::NUMBER);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mod1::mod2;

    #[test]
    fn test_pub() {
        mod2::print_number();
    }
}
