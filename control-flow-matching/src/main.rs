fn main() {
    let n = 6;
    if n % 4 == 0 {
        println!("n is divisible by 4")
    } else if n % 3 == 0 {
        println!("n is divisible by 3")
    } else if n % 2 == 0 {
        println!("n is divisible by 2")
    } else {
        println!("n is not divisible by 4,3, or 2")
    }

    //or
    let c = true;

    let m = if c { 50 } else { 76 };

    println!("n {}", m);

    //loops while, for and loop
    let mut d = 0;

    loop {
        println!("finite");

        d += 1;

        if d < 10 {
            break;
        }
    }

    //match
    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("tow"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    let w = 5;

    match w {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
        13...19 => println!("a teen"),
        _ => println!("ain't special"),
    }

    let v = (5, -5);

    match v {
        (x, y) if x == y => println!("equal"),
        (x, y) if x + y == 0 => println!("equal zero"),
        (x, _) if x % 2 == 0 => println!("X is even"),
        _ => println!("No match"),
    }
}
