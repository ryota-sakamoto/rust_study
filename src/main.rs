struct Object {
    name: String,
}
impl Object {
    fn new(name: &str) -> Object {
        Object {
            name: name.to_string(),
        }
    }

    fn call(&self) {
        println!("call run {}", self.name);
    }
}

fn main() {
    let obj = Object::new("name1");
    obj.call();
    println!("{}", obj.name);
}