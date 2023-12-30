fn main() {
    let mut x = vec![1]; // Create vector x
    let y = &x;         // x loses write/ownership
    // print_num(x);               // Can't pass x because it's borrowed

    let z = (*y)[0];            // End of y, x regains write/ownership
    x[0] += z;

    // print_num(x);
    print_num2(&x);
}

fn print_num(i: Vec<i32>){
    println!("#{}", i[0]);
}

fn print_num2(i: &Vec<i32>){
    println!("#{}", i[0]);
}
