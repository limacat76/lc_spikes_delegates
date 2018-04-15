fn l<X: GlobalT>(a_name : &'static str, a_function : fn(&mut X), a_global : &mut X) {
    println!("Running {} on {}", a_name, GlobalT::get_name(a_global));
    a_function(a_global);
}

trait GlobalT {
    fn get_name(&self) -> std::string::String;
}

/*
struct GlobalG<X: GlobalT> {
    global : X
}
*/

#[cfg(test)]
mod tests {
    use std;
    use GlobalT;
    use l;

    trait TestAddsT : GlobalT {
        fn add1ToValue(&mut self);
        fn getValue(&self) -> i32;
    }

    struct TestAddsS {
        index : i32
    }

    fn get_test_adds() -> TestAddsS {
        TestAddsS {
            index : 0,
        }
    }

    impl GlobalT for TestAddsS {
        fn get_name(&self) -> std::string::String {
            String::from("TestAddsS")
        }
    }

    impl TestAddsT for TestAddsS {

        fn add1ToValue(&mut self) {
            self.index = self.index + 1;
        }

        fn getValue(&self) -> i32 {
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
        tt.add1ToValue();
        assert_eq!(tt.getValue(), 1);
    }

    fn delegate_adder(tt : &mut TestAddsS){
        tt.add1ToValue();
    }

    #[test]
    fn it_adds_delegate() {
        let mut tt = get_test_adds();
        tt.add1ToValue();
        l("add_delegate", delegate_adder, &mut tt);
        assert_eq!(tt.getValue(), 2);
    }

}
