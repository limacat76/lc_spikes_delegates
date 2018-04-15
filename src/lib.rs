pub fn l<Specialized: Environment>(a_name : &'static str, a_function : fn(&mut Specialized), an_environment : &mut Specialized) {
    println!("Running {} on {}", a_name, Environment::get_name(an_environment));
    a_function(an_environment);
}

pub trait Environment {
    fn get_name(&self) -> std::string::String;
}

#[cfg(test)]
mod tests {
    use std;
    use Environment;
    use l;

    trait TestAddsT : Environment {
        fn add_1_to_value(&mut self);
        fn get_my_value(&self) -> i32;
    }

    struct TestAddsS {
        index : i32
    }

    fn get_test_adds() -> TestAddsS {
        TestAddsS {
            index : 0,
        }
    }

    impl Environment for TestAddsS {
        fn get_name(&self) -> std::string::String {
            String::from("TestAddsS")
        }
    }

    impl TestAddsT for TestAddsS {

        fn add_1_to_value(&mut self) {
            self.index = self.index + 1;
        }

        fn get_my_value(&self) -> i32 {
            self.index
        }

    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_adds_plain() {
        let mut tt = get_test_adds();
        tt.add_1_to_value();
        assert_eq!(tt.get_my_value(), 1);
    }

    fn delegate_adder(tt : &mut TestAddsS){
        tt.add_1_to_value();
    }

    #[test]
    fn it_adds_delegate() {
        let mut tt = get_test_adds();
        tt.add_1_to_value();
        l("add_delegate", delegate_adder, &mut tt);
        assert_eq!(tt.get_my_value(), 2);
    }

}
