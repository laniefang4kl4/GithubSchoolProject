fn main() {
    let mut number = 0;
    println!("Enter a number: ");
    io::stdin().read_line(&mut number);
    
    if number % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
