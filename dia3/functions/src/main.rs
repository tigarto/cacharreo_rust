fn main() {
    println!("Hello, world!");

    another_function1();
    another_function2(5);
    print_labeled_measurement(5, 'h');
    // ---
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    // ----
    let x = five();
    println!("The value of x is: {x}");
    // ---
    let x = plus_one(5);
    println!("The value of x is: {x}");
    // ---
    println!("max({x},{y}) = {}", max(x, y));
    // ---
    let marks = 56;
    print_grade(marks);
}

fn another_function1() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn print_grade(marks: u8) {
    if marks >= 90 {
        println!("Grade is A");
    } else if marks >= 80 {
        println!("Grade is A-");
    } else if marks >= 70 {
        println!("Grade is B");
    } else if marks >= 60 {
        println!("Grade is B-");
    } else if marks >= 50 {
        println!("Grade is C");
    } else if marks >= 40 {
        println!("Grade is C-");
    } else if marks >= 30 {
        println!("Grade is D");
    } else {
        println!("Grade is F");
    }
}
