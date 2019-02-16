
fn main() {
    print_num(10);
}

fn print_num(num:u32) {

    for i in 0..num {
        if is_even(i) {
            println!("{} is even!",i);
        }
        else {
            println!("{} is odd!",i);
        }
    }
}

fn is_even(num:u32) -> bool {
    return num % 2 == 0;
}