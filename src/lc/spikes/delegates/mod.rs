use lc::spikes::contexts::Environment;

pub fn l<Specialized: Environment>(a_name : &'static str, a_function : fn(&mut Specialized), an_environment : &mut Specialized) {
    println!("Running {} on {}", a_name, Environment::get_name(an_environment));
    a_function(an_environment);
}