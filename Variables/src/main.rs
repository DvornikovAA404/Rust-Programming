fn main() {
    let x: i8 = 127;
    println!("The value x: {x}");

    let tup = (20, 'f', 19);
    let (x, y, z) = tup;
    println!("The tuple is {y}");

    let a = [[1, 3, 2], [2, 5, 9], [9, 9, -7]];
    let y = a[0][2];
    println!("The array is {y}");
}
