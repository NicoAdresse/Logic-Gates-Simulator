use std::io;

// Helper Functions
fn and(a: bool, b: bool) -> bool { a && b }
fn or(a: bool, b: bool) -> bool { a || b }
fn not(a: bool) -> bool { !a }
fn xor(a: bool, b: bool) -> bool { a ^ b }
fn nand(a: bool, b: bool) -> bool { !(a && b) }
fn nor(a: bool, b: bool) -> bool { !(a || b) }
fn xnor(a: bool, b: bool) -> bool { a == b }

// Get Input Function
fn get_input(prompt: &str) -> bool {
    println!("{}", prompt);
    let mut insert: String = String::new();

    io::stdin()
        .read_line(&mut insert)
        .expect("Error: Failed to read input.");

    let input = insert
        .trim()
        .parse()
        .expect("Error: Parsing failed.");

    match input {
        0 => false,
        1 => true,
        _ => panic!("Error: Invalid input."),
    }
}

// Logic Functions

// AND Gate
fn run_and_gate() {
    let x: bool = get_input("Input 1 (0/1): ");
    let y: bool = get_input("Input 2 (0/1): ");
    println!("AND result: {}.", and(x, y));
}

// OR Gate
fn run_or_gate() {
    let x: bool = get_input("Input 1 (0/1): ");
    let y: bool = get_input("Input 2 (0/1)");
    println!("OR result: {}.", or(x, y));
}

// NOT Gate
fn run_not_gate() {
    let x: bool = get_input("Input (0/1): ");
    println!("NOT result: {}.", not(x));
}

// XOR Gate
fn run_xor_gate() {
    let x: bool = get_input("Input 1 (0/1): ");
    let y: bool = get_input("Input 2 (0/1): ");
    println!("XOR result: {}.", xor(x, y));
}

// NAND Gate
fn run_nand_gate() {
    let x: bool = get_input("Input 1 (0/1): ");
    let y: bool = get_input("Input 2 (0/1): ");
    println!("NAND result: {}.", nand(x, y));
}

// NOR Gate
fn run_nor_gate() {
    let x: bool = get_input("Input 1 (0/1): ");
    let y: bool = get_input("Input 2 (0/1): ");
    println!("NOR result: {}.", nor(x, y));
}

// XNOR Gate
fn run_xnor_gate() {
    let x: bool = get_input("Input 1 (0/1): ");
    let y: bool = get_input("Input 2 (0/1): ");
    println!("XNOR result: {}.", xnor(x, y));
}

// Main
fn main() {
    println!("0. Exit\n    1. AND / 2. OR / 3. NOT\n    4. XOR / 5. NAND / 6. NOR / 7. XNOR\nChoice: ");
    loop {
        let mut insert: String = String::new();

        io::stdin()
            .read_line(&mut insert)
            .expect("Error: Failed to read input.");

        let input: u8 = insert
            .trim()
            .parse()
            .expect("Error: Parsing failed.");

        if input == 0 {
            break;
        }

        if input == 1 {
            run_and_gate();
        } else if input == 2 {
            run_or_gate();
        } else if input == 3 {
            run_not_gate();
        } else if input == 4 {
            run_xor_gate();
        } else if input == 5 {
            run_nand_gate();
        } else if input == 6 {
            run_nor_gate();
        } else if input == 7 {
            run_xnor_gate();
        }
    }
}
