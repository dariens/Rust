
const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("{}", MAX_POINTS);

    let x = x + 1;

    let x = x * 2;

    println!("{}", x);

    let spaces = "    ";
    let spaces = spaces.len();

    println!("{}", spaces );

    let x = 2.0;
    let y: f32 = 3.0;

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (a, b, c) = tup;

    println!("{}, {}, {}", a, b, c);

    let five_hundred = tup.0;
    println!("{}", five_hundred);

    let a = [1,2,3,4,5];

    let first = a[0];
    let second = a[1];
}
