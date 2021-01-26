fn main() {
    let address = 0o32;
    let list = [1, 2, 5, 26, 42, 132, 3412];

    for item in &list {
        if *item == address {
            println!("{}", item)
        }
    }

    println!("{}", address);
}
