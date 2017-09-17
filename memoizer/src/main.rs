use std::collections::HashMap;

struct Cacher<T>
    where T: Fn(i32) -> i32 {
        calculation: T,
        value: HashMap<i32, i32>,
}

impl<T> Cacher<T>
    where T: Fn(i32) -> i32 {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: HashMap::new()
            }
        }

        fn value(&mut self, param: i32) -> i32 {
            let result = self.value.entry(param).or_insert((self.calculation)(param));
            return *result
        }
}

fn main() {
    let mut cacher = Cacher::new(|x| x + 1);

    println!("Value: {}", cacher.value(3));
    println!("Value: {}", cacher.value(4));

}
