# Dia 2

## Actividades 

- [ ] Capitulo 2 del libro ([Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html))


### Actividad 2 - Programming a Guessing Game

Antes de empezar vamos a tener a la mano la referencia: [Rust Language Cheat Sheet](https://cheats.rs/)

Aqui se veran algnos conceptos básicos que se usaran cuando se codifica un programa:
* ```&```
* ```let```
* ```mut```
* ```match```
* ```println!```
* ```String```:
  * ```String::new```
  * 
* Metodos
* Funciones asociadas.
* Uso de dependencias externas (external crates): ```use```
  * ```std```
    *  ```Result```
       * ```expect```   
  * ```std::io```:
    * ```io::stdin()```
      *  ```read_line```         
  * ```rand::Rng```
    *  ```rand::thread_rng```
       *   ```gen_range```  
  * ```std::cmp::Ordering``` 
    * ```Ordering```
      *  ```Less``` 
      *  ```Greater``` 
      *  ```Equal``` 


**Problema - Juego de adivinanzas**

Un programa genera un número entero aleatorio entre 1 y 100 para luego pedir al jugador adivine el valor ingresando el dato por teclado. Después de que el jugador ingresa el valor, el programa indicará si el valor esta por encima o por debajo. Si el jugador le atina al valor, el juego imprimirá un mensaje de felicitación y saldrá.

**Pasos**

1. Configuración del proyecto

```
$ cargo new guessing_game
$ cd guessing_game
```

2. Test inicial

```
$ cargo run
```

3. Codificación del programa:

* **Archivo**: [main.rs](./guessing_game/src/main.rs)

```rs
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

**Sobre la librerias**:

|Libreria|nombre|Comentarios|
|---|---|---|
|Estandar|```std```||
|I/O|```std::io```|Modulo ```io``` Se encuentra dentro de ```std```|
|rand|```rand::Rng```|Se debe agregar en el archivo **Cargo.toml** la ```rand``` crate como dependencia (```rand = "0.8.3"```).|
||```std::cmp::Ordering```||

Para actualizar las librerias que esta usando use ```cargo update```, con esto se .

**Salida en pantalla**:

Se usa ```println!``` la cual es una macro que imprime un string en pantalla.

Mediante el uso de las llaves se puede imprimir el valor de una variable.

```rs
println!("You guessed: {guess}");
```

Para imprimir mas de una variable, puede basarse en el fragmento de codigo mostrado a continuación:

```rs
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

**Declaración de variables**:

Para declara variables se usa ```let```. En Rust, las variables son inmutables por defecto (una vez creadas no pueden cambiar de valor). Para hacer que estas puedan modificarse se debe agregar ```mut``` antes del nombre de la variable.

```rs
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

Para crear una variable tipo ```String``` (Mas exactamente una instancia tipo ```String```), ```new``` es una **función asociada** a un tipo (en este caso String); para este caso parece que fuera algo como el metodo constructor o algo asi ```???```. El ```::``` indica a lo que se asocia el la función.

```rs
let mut guess = String::new();
```

**Entrada de datos**:

Se usa la función ```stdin``` que se encuentran dentro del modulo ```io``` de ```std``` , la parte ```read_line(&mut guess)``` llama al metodo ```read_line```, al usar ```&mut guess``` decimos donde se almacenará lo que se ingresa por teclado. El ```&``` significa que el argumento pasado es una referencia. Se escribe ```&mut guess``` en vez de ```&guess``` para indicar que la variable es mutable.

```rs
io::stdin()
    .read_line(&mut guess)
```

Como puede haber una potencial falla cuando de acuerdo al tipo de resultado se agrega:

```rs
.expect("Failed to read line");
```

Asi, la expresión completa para entrar un dato por teclado queda:

```rs
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

Pero lo ideal es escribir la espresión dividiendola, usando **```.method_name()``` syntax**

**Generación de numeros aletorios**:

Rust no incluye la funcionalidad para la generación de numeros aleatorios en su libreria estandar. En vez de ello, proporciona una **rand crate** con esta funcionalidad.

>
> **crate**<br> 
> Es una colección de archivos fuente de rust. El **rand crate** es una libreria crate la cual contiene codigo que será usado por otros programas. 
> 

```rs
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

**Generando la documentación del programa**

```
cargo doc --open
```

![doc_rust_project](doc_game.png)

**Realizando comparaciones**

```rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

El corazon de la comparación esta en hacer uso de ```match```, una **expresión match** o expresión a comparar hace referencia al patrón la que se encuentra entre las llaves ```{expresion}``` y que sera usado para la comparación (**Psd**: No se si esto esta bien traducido o es fiel a lo que se quiere decir en ingles).

Aqui hay un ejemplo de uso:

```rs
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Finalmente, antes de dejar el codigo funcional, es necesario convertir la variable ```guess``` de String entero lo cual se hace con la siguiente instrucción (para mas información ver [parse](https://doc.rust-lang.org/std/primitive.str.html#method.parse)):

```rs
let mut guess = String::new();

// Notese que guess ya se habia definido
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Rust a diferencia de C permite por asi decirlo redefinir variables que previamente se han definido (algo similar a lo que pasa en python con una variable que en un momento es de un tipo y luego esa misma es de otro), lo cual permite ahorrarse el uso de tener que declarar una nueva variable (Tal ocmo ```guess_str``` y ```guess``` por ejemplo). Ahora si retornando a la expresión de interes tenemos:

```rs
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

En esta, lo que se hace es asociar a la nueva variable la expresión ```guess.trim().parse()```. Aqui, ```guess``` se refiere a la variable original, el string leido, al cual se le aplica el método en el cual metodo ```trim``` para eliminar los espacios en blanco al principio y al final del string,  con el método ```parse``` se realiza la conversión de tipo, y finalmente mediante el ```expect``` se hace la verificación del tipo, haciendo que se saque el mensaje **Please type a number!** cuando el programa se sale por un error cuando no se pone a la entrada un entero. 

---- Aca vamos ----

**Realizando comparaciones**


## Codigo final

```rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## Referncias

* https://prev.rust-lang.org/es-ES/documentation.html
* https://reberhardt.com/cs110l/spring-2020/
* https://events.static.linuxfound.org/sites/events/files/slides/rust-4-cpp2.pdf
* https://static.linaro.org/connect/lvc21f/presentations/LVC21F-317.pdf
* https://docs.google.com/presentation/d/1Xrzj8Ul690fJr6MjfNWOiF2mPKsVwX1g4FCLtOT-vdQ/edit#slide=id.g4b027f704f_1_174
