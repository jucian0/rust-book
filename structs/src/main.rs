#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Object {
        Object { width, height }
    }

    fn show(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}

fn main() {
    let obj1 = Object {
        height: 35,
        width: 35,
    };

    let obj2 = Object::new(37, 40);

    obj1.show();
    obj2.show();

    println!("{:?}", obj1);
    println!("{:?}", obj2);
}
