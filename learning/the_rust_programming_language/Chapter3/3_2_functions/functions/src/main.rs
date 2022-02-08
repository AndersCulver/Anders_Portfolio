fn main() {

    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("The measurement is: {}{}", x, unit_label);
}
