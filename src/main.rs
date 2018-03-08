use std::ops::Add;

#[derive(Debug, Clone)]
struct Object {
    name: String,
    count: u64,
}
impl Object {
    fn new(name: &str, count: u64) -> Object {
        Object {
            name: name.to_string(),
            count: count,
        }
    }

    fn call(&self) {
        println!("call run {}", self.name);
    }

    fn change_name(&self, name: &str) {
        // self.name = name.to_string();
    }
}

impl Add for Object {
    type Output = Object;

    fn add(self, other: Object) -> Object {
        Object {
            name: self.name,
            count: self.count + other.count,
        }
    }
}

fn main() {
    let obj = Object::new("name1", 5);
    obj.call();
    println!("{:?}", obj);

    let obj2 = Object::new("name2", 10);
    let obj3 = obj.clone();
    println!("{:?}", obj2 + obj3);
    // println!("{:?} {:?}", obj2, obj3);

    let s1 = &obj.name;
    let s2 = obj.name.as_str();
    assert_eq!(s1, s2);

    println!("{}", obj.name.chars().count());
    println!("{}", obj.name.as_str().chars().count());

    let array = ["1", "2", "3"];
    println!("{} {:?}", array.join("_"), array);

    obj.change_name("name2");

    let option_some: Option<u32> = Some(1);
    let option_none: Option<u32> = None;

    let option_some_map = option_some.map(|number| {
        number + 1
    });
    let option_none_map_or_else = option_none.map_or_else(
        || {
            println!("none");
            Some(0)
        },
        |number| Some(number + 1)
    );
    println!("{:?} {:?}", option_some_map, option_none_map_or_else);

    let func: Box<Fn(u64) -> u64> = Box::new(|a| a.pow(2));
    println!("{:?}", func(5));

    let v = vec![1, 2, 3, 4, 5];
    for n in &v {
        print!("{}", n);
    }
    println!("{:?}", v);
}