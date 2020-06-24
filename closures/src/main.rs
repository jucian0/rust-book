fn receives_closure<F>(closure: F)
where
    F: Fn(i32) -> i32,
{
    let result = closure(1);
    println!("closure(1) => {}", result);
}

fn return_closures() -> impl Fn(i32) -> i32 {
    |x| x + 4
}

fn main() {
    let closure = return_closures();
    receives_closure(closure);

    let y = 2;
    // let add = |x| x + y;
    receives_closure(|x| x + y);
}
