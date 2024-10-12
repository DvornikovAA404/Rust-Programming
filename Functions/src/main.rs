fn main() {
    println!("Hello, world!");
    let mut x = 5;
    let y = 65;
    let mut k = 0;
    loop{
        k += 1;
        print!("{k}    {x}+{y}=");
        x = sum_f(x, y);
        println!("{x}");
        if x > 3000{
            break;
        }
    }
}

fn sum_f(x: i32, y: i32) -> i32{
    let z = x + y;
    return z;
}
