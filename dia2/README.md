# Dia 2

## Actividades 

- [ ] Capitulo 2 del libro ([Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html))


### Actividad 2 - Programming a Guessing Game

Antes de empezar vamos a tener a la mano la referencia: [Rust Language Cheat Sheet](https://cheats.rs/)

Aqui se veran algnos conceptos básicos que se usaran cuando se codifica un programa:
* ```let```
* ```match```
* Metodos
* Funciones asociadas.
* Uso de dependencias externas (external crates)

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
|---|---||
|Estandar|```std```||
|I/O|```std::io```|Modulo ```io``` Se encuentra dentro de ```std```|


**Salida en pantalla**:
Se usa ```println!``` la cual es una macro que imprime un string en pantalla.

**Declaración de variables**

Para declara variables se usa ```let```. En Rust, las variables son inmutables por defecto (una vez creadas no pueden cambiar de valor). Para hacer que estas puedan modificarse se debe agregar ```mut``` antes del nombre de la variable.

```rs
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

Para crear una variable tipo ```String``` (Mas exactamente una instancia tipo ```String```), ```new``` es una **función asociada** a un tipo (en este caso String); para este caso parece que fuera algo como el metodo constructor o algo asi ```???```. El ```::``` indica a lo que se asocia el la función.

```rs
let mut guess = String::new();
```

**Entrada de datos**

Se usa la función ```stdin``` que se encuentran dentro del modulo ```io``` de ```std``` , la parte ```read_line(&mut guess)``` llama al metodo ```read_line```, al usar ```&mut guess``` decimos donde se almacenará lo que se ingresa por teclado. El ```&``` significa que el argumento pasado es una referencia. Se escribe ```&mut guess``` en vez de ```&guess``` para indicar que la variable es mutable.

```rs
io::stdin()
    .read_line(&mut guess)
```

Siguiendo con lo demás,




we’ll create a variable to store the user input, like this:

let mut guess = String::new(); 

We use the let statement to create the variable.

In Rust, variables are immutable by default, meaning once we give the variable a value, the value won't change.
To make a variable mutable, we add mut before the variable name:



use rand::Rng;


println! is a macro that prints a string to the screen:

Creación de variables



To obtain user input and then print the result as output, we need to bring the io input/output library into scope. The io library comes from the standard library, known as std:


https://prev.rust-lang.org/es-ES/documentation.html
https://reberhardt.com/cs110l/spring-2020/
https://events.static.linuxfound.org/sites/events/files/slides/rust-4-cpp2.pdf
https://static.linaro.org/connect/lvc21f/presentations/LVC21F-317.pdf
https://docs.google.com/presentation/d/1Xrzj8Ul690fJr6MjfNWOiF2mPKsVwX1g4FCLtOT-vdQ/edit#slide=id.g4b027f704f_1_174
