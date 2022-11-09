# Dia 3

## Actividades 

- [ ] Capitulo 3 del libro: ([Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html))
   - [ ] Variables y Mutabilidad ([link](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html))
      - [ ] Apuntes
      - [x] Analisis de Código ([variables](./variables/))
   - [ ] Tipos de datos ([link](https://doc.rust-lang.org/book/ch03-02-data-types.html))
      - [ ] Apuntes
      - [x] Analisis de Código ([data_types](./data_types/))
   - [ ] Funciones ([link](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html))
      - [ ] Apuntes
      - [x] Analisis de Código
   - [ ] Comentarios ([link](https://doc.rust-lang.org/book/ch03-04-comments.html))
      - [ ] Apuntes
      - [ ] Analisis de Código
   - [ ] Control de flujo ([link](https://doc.rust-lang.org/book/ch03-05-control-flow.html))
      - [ ] Apuntes
      - [ ] Analisis de Código


# Actividad 3 - Common Programming Concepts

Los conceptos aqui mostrados son comunes a todos los lenguajes de programación; sin embargo, se discutiran como llevarlos a cabo en el contexto de Rust. Tomando como base el libro se cubrira:
* Variables
* Tipos básicos de datos
* Funciones
* Comentarios
* Control de flujo

## Palabras claves (keywords)

Son palabras que estan reservadas para su uso en Rust de manera exclusiva y solo por lo tanto no pueden ser usadas como nombres de variables o funciones. En el siguiente [link](https://doc.rust-lang.org/book/appendix-01-keywords.html) se encuentran las reservadas en Rust.

## Variables y mutabilidad

* **Antes de empezar**

Se inicio y se avanzo en el tutorial empezando la ejecución de los siguientes comandos:

```
cargo new variables
cd variables
```

* **Referencia**: [Variables and mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

En Rust las variables por default son inmutables (para aprovechar la seguridad y la concurrencia). Una variable **inmutable** se caracteriza por que una vez que un valor se vinvula a un nombre (de variable), este valor no puede ser cambiado. Por ejemplo:

```rs
let x = 5;
x = 6; // Error: An immutable variable was reassigned (rustc --explain E0384)
```

Un concepto que se introduce en Rust es la **mutabilidad**. Aunque las variables son inmutables por default, es posible hacerlas mutables al agregar la palabra clave ```mut``` antes del nombre de la variable.

```rs
let mut x = 5;
x = 6; // OK, ahora si es posible cambiar el valor de la variable
```

Cuando se indica que una variable es **mut**, se esta indicando la intención a otro programador que esta
variable estará sometida a cambios en otras partes del código. Es importante anotar que una constante **siempre sera inmutable**

### Constantes

Al igual que las variables inmutables, las constantes son valores una vez se vinculan a un nombre y pueden ser cambiados. Es importante tener en cuenta los siguientes apuntes:
1. No se permiten usar ```mut``` con constantes.
2. Si se declara una constante usando la palabra clave ```const``` en vez de ```let```, es necesario indicar el tipo de dato ([link](https://doc.rust-lang.org/book/ch03-02-data-types.html#data-types)) asociado a la constante.
3. Las constantes pueden ser declaradas en cualquier ambito (scope) incluyento el ambito global.
4. Las constnates solo puede ser una expresión constante, no el resultado una expresión cuya evaluación se realiza en tiempo de ejecución.
5. Las constantes son validas, dentro del ambito del cual fueron declaradas, durante el tiempo que el programa se ejecuta. 

En Rust, la convención de nombrado de constantes es usar todas las palabras en mayusculas con underscores (guión bajo: '''_''') entre palabras. A contiación se declara una constante:

```rs
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### Shadowing

El **variable shadowing** (algo como ocultamiento) ocurre cuando una variable declarada dentro de cierto ambito (bloque de decisión, metodo, or clase interna) tiene el **mismo nombre** de una variable que se declaro en un ambito mas externo. En estos palabras se suele decir que la variable externa suele ser shadowed (ocultada) por la interna, de modo que el valor desplegado para la variable (que se llama igual) será el del ambito de exitencia mas interno.

```rs
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// -- prints --
// The value of x in the inner scope is: 12
// The value of x is: 6
```

**Disculpas**: Aun no hemos acabado pero planeamos hacerlo... :shipit:


## Tipos de datos

Se inicio y se avanzo en el tutorial empezando la ejecución de los siguientes comandos:

```
cargo new data_types
cd data_types
```

En el directorio que se crea: [data_types](./data_types/) se encuentra el codigo fuente de los ejemplos analizados en [main.rs](../dia3/data_types/src/main.rs)


```rs
let guess: u32 = "42".parse().expect("Not a number!");
```

| Length | Signed | Unsigned |
|---|---|---|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| Architecture-specific (32/64) | isize | usize |


**Disculpas**: Aun no hemos acabado pero planeamos hacerlo... :shipit:

## Funciones

Se inicio y se avanzo en el tutorial empezando la ejecución de los siguientes comandos:

```
cargo new functions
cd functions
```

En el directorio que se crea: [functions](./functions/) se encuentra el codigo fuente de los ejemplos analizados en [main.rs](./functions/src/main.rs)

### Apuntes a la loca...

* La función ```main``` es el punto de entrada.
* Rust usa el **snake case** [link](https://en.wikipedia.org/wiki/Snake_case) como estilo convensional para funciones y nombres de variables. En este, todas las letras con escritas en minusculas y las palabras se separan usando guión bajo. 

```rs
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

### Statements and expressions

* El cuerpo de las funciones esta hecho de una serie de sentencias (statements) y opcionalmente terminan en una expresion (expression).
* Al ser Rust un **expression-based language** ([link](https://en.wikipedia.org/wiki/Expression-oriented_programming_language#:~:text=An%20expression%2Doriented%20programming%20language,languages%20often%20treat%20as%20statements.)) es importante distinguir entre **statement** y **expression**:
  * **Statement**: Son instrucciones que realizan alguna tarea sin retornar un valor.
    * Instrucciones dentro una funcion.
  
      ```rs
      fn main() {
         let y = 6; // Statement
      }
      ```

    * Definiciones de funciones (Cuerpo completo del ejemplo anterior). 
  * **Expression**: Son instrucciones que al evaluarse retornan un valor.
    * Las expresiones pueden ser parte de statements.
    * Llamar a una funcion es una expresion.
    * Llamar a una macro es una expresión.
    * Un nuevo bloque creado entre llaves es una expresión:
      
      ```rs
      fn main() {
         let y = {
            let x = 3;
            x + 1  // Importante, no hay punto y coma al final de una expresion
         }; 
         // y = 4

         /*
         La expresion: 
         {
            let x = 3;
            x + 1
         }
         */
         
         println!("The value of y is: {y}");
      }
      ```


* **Sintaxis de una función**

  * **Cuando la función no retorna**:
  
     * **Definición**:
  
        ```rs
        fn function_name(param1: type1, param2: type2,...) {
           // Code
        }
        ```

     * **Invocación**:

        ```rs
        function_name(arg1, arg2,...);
        ```

  * **Cuando la función retorna**:
     
     * **Definición**:
  
        ```rs
        fn function_name(param1: type1, param2: type2,...) -> return_type {
           // Code
        }
        ```

     * **Invocación**:

        ```rs
        var_name = function_name(arg1, arg2,...);
        ```


* **Ejemplos de funciones**:
  
  * **Sin parametros y sin retorno**:

     ```rs
     fn function_hello() {
        println!("Hello World");
     }
     ```

  * **Con parametros y sin retorno**:

     ```rs
     fn another_function(x: i32) {
        println!("The value of x is: {x}");
     }

     fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
     }
     ```
   * **Sin parametros y con retorno**:
     ```rs
     fn foo() -> bool { 
        true 
     }

     fn five() -> i32 {
        5
     }
     ```
  
   * **Con parametros y con retorno**:

     ```rs     
     fn plus_one(x: i32) -> i32 {
        x + 1
     }     
     ```

**Disculpas**: Aun no hemos acabado pero planeamos hacerlo... :shipit:

## Comentarios

Permiten documentar

**Disculpas**: Aun no hemos acabado pero planeamos hacerlo... :shipit:

## Control de flujo

**Disculpas**: Aun no hemos acabado pero planeamos hacerlo... :shipit:

## Para ensayar luego

1. https://runmycode.online/
2. https://github.com/judge0/judge0
3. https://visualstudiomagazine.com/articles/2021/08/31/github-vs-code.aspx
4. https://stackoverflow.com/questions/6551446/can-i-run-html-files-directly-from-github-instead-of-just-viewing-their-source
5. https://raw.githack.com/ (ver)
6. https://www.gitpod.io/





