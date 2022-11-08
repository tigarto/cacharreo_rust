use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Data types");

    // ****** Scalar Types ******
    println!("---- Scalar Types ----");
    // Integer Types
    let a: i32 = -10_000; // usa del separador visual 
    let b: u32 = 10000;   // no se usa el separador visual 
    let mut c: u8 = 0xFF;     // Hexadecimal
    println!("Integer Types");
    println!("-> a = {a}");
    println!("-> b = {b}");
    println!("-> c = {c}");
    // c += 1; // Si se descomenta sale: attempt to add with overflow -> Ejercucion: cargo run 
               // Descomente la linea anteriore e intente: cargo run --release
    println!("c = {c}");

    // Floating-Point Types
    println!("Floating-Point Types");
    let x = 2.0; // f64 (Double precision)
    let y: f32 = 3.0; // f32 (Single precision)
    println!("-> x = {x}");
    println!("-> y = {y}");

    // Numeric Operations
    println!("Numeric Operations");
    // addition
    let sum = 5 + 10;
    println!("-> 5 + 10 = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("-> 95.5 - 4.3 = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("-> 4 * 30 = {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("-> 56.7 / 32.2 = {quotient}");
    let floored = 2 / 3; // Results in 0
    println!("-> 2 / 3 = {floored}");

    // remainder
    let remainder = 43 % 5;
    println!("-> 43 % 5 = {remainder}");

    // Boolean Type
    println!("Boolean Type");
    let t = true;
    let f: bool = false; // with explicit type annotation    
    println!("-> t = {t}");
    println!("-> f = {f}");

    // Character Type
    println!("Character Type");
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);
    println!();
    println!("---- Compound types ----");

    // Tuple Type
    println!("Tuple Type");
    let tup: (i32, f64, u8) = (500, 6.4, 1);    
    println!("-> tup = ({}, {}, {})", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;   // destructuring
    println!("-> (x,y,z) = ({x}, {y}, {z})");
    let tup2 = (500, 6.4, 1); 
    let five_hundred = tup2.0; // Uso del punto para acceder
    let six_point_four = tup2.1;
    let one = tup2.2;
    println!("-> tup2 = ({five_hundred}, {six_point_four}, {one})");
    let tup_empty = ();  // unit: Tupla sin valores
    // println!("tup_empty = ({})",tup_empty);

    // Array Type
    println!("Array Type");
    let a = [1, 2, 3, 4, 5];
    println!("-> a = [{}, {}, {}, {}, {}]", a[0], a[1], a[2], a[3], a[4]);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("-> Yisus nace in {}", months[11]);

    let a: [i32; 5] = [6, 7, 8, 9, 10];
    println!("-> a = [{}, {}, {}, {}, {}]", a[0], a[1], a[2], a[3], a[4]);

    let a = [3; 5];
    println!("-> a = [{}, {}, {}, {}, {}]", a[0], a[1], a[2], a[3], a[4]);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("-> a[0] = {first}, a[1] = {}", a[1]);

    // Ultimo codigo ejemplo
    let a = [10, 20, 30, 40, 50];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    /* No se valida el tamaño
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
    */ 
    
    const SIZE_A: usize = 5;
    
    match index.cmp(&SIZE_A){
        Ordering::Less => {
            let element = a[index];
            println!("The value of the element at index {index} is: {element}");
        },
        Ordering::Greater => println!("Error"),
        Ordering::Equal => println!("Error")
    };
}
