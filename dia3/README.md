# Dia 3

## Actividades 

- [ ] Capitulo 3 del libro: ([Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html))
   - [ ] Variables y Mutabilidad ([link](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html))
   - [ ] Tipos de datos ([link](https://doc.rust-lang.org/book/ch03-02-data-types.html))
   - [ ] Funciones ([link](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html))
   - [ ] Comentarios ([link](https://doc.rust-lang.org/book/ch03-04-comments.html))
   - [ ] Control de flujo ([link](https://doc.rust-lang.org/book/ch03-05-control-flow.html))


## Actividad 3 - Common Programming Concepts

Los conceptos aqui mostrados son comunes a todos los lenguajes de programación; sin embargo, se discutiran como llevarlos a cabo en el contexto de Rust. Tomando como base el libro se cubrira:
* Variables
* Tipos básicos de datos
* Funciones
* Comentarios
* Control de flujo

### Palabras claves (keywords)

Son palabras que estan reservadas para su uso en Rust de manera exclusiva y solo por lo tanto no pueden ser usadas como nombres de variables o funciones. En el siguiente [link](https://doc.rust-lang.org/book/appendix-01-keywords.html) se encuentran las reservadas en Rust.

### Variables y mutabilidad

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





