fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut x = 10;
    while x < 20 {
        x = x + 1;
        println!("{}", x);
    }

    let array = [1, 2, 3, 4];

    for a in array.iter() {
        println!("{}", a);
    }
}

fn add() {

}
