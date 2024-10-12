fn main() {
    println!("Hello, world!");
    let mut x = 5;
    let mut y = 65;
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
    println!("------");
    let mut i = 0;
    let mut j = 0;
    y = 5;
    x = 7;
    while i < y{
        j = 0;
        while j < x{
            j += 1;
            print!("{i}  ");
        }
        i += 1;
        println!("!");
    }
    println!("------");
    i = 0;
    j = 0;
    let a = 28;
    let b = a/3;
    for line in (0..a).rev(){
        i = 1+(line*2);
        j = ((a*2-1)-i)/2;
        for sp in (0..j){
            print!(" ")
        }
        if i > b*2{
            for v in (0..b){
                print!("v");
            }
            
                for sp in (0..(i-(b*2))){
                    print!(" ");
                }    
                for v in (0..b){
                    print!("v");
                }   
            }
        else{
            for v in (0..i){
                print!("v");
            }
        } 
        println!();
    }
}

fn sum_f(x: i32, y: i32) -> i32{
    let z = x + y;
    return z;
}
