// ============================================================
// CONCEPTO: Variables y mutabilidad
// ============================================================
// En Rust todas las variables son inmutables por defecto.
// Para permitir cambio de valor usa `mut`.
//
// EJEMPLO:
//   let x = 5;        // inmutable
//   let mut y = 5;    // mutable
//   y = 10;           // válido
//   x = 10;           // ERROR: cannot assign twice
//
// SHADOWING:
// Puedes redeclarar una variable con `let` (crea nueva binding):
//   let x = 5;
//   let x = x + 1;    // válido: nueva variable
//   let x = "texto";  // válido: puede cambiar de tipo
//
// ANALOGÍA C:
//   const int x = 5;  ≈  let x = 5;
//   int y = 5;        ≈  let mut y = 5;
//
// DIFERENCIA CLAVE: En C no hay shadowing. En Rust sí.
// ============================================================

// EJERCICIO 1: Arregla este código para que compile
// Hay dos formas: usar `mut` o usar shadowing
fn ejercicio_1() {
    let mut x = 5;
    println!("El valor inicial de x es: {}", x);
    x = 10; // error: cannot assign twice to immutable variable
    println!("El nuevo valor de x es: {}", x);
}

// EJERCICIO 2: Declara una variable mutable, cámbiala dos veces
// Usa `mut` para este ejercicio
fn ejercicio_2() {
    // Declara una variable mutable `temperatura` con valor inicial 20
    // TODO: tu código aquí
    let mut temperatura = 20;
    println!("Temperatura inicial: {}°C", temperatura);

    // Aumenta la temperatura a 25
    // TODO: tu código aquí
    temperatura = 25;
    println!("Nueva temperatura: {}°C", temperatura);

    // Aumenta la temperatura a 30
    // TODO: tu código aquí
    temperatura = 30;

    println!("Temperatura final: {}°C", temperatura);
}

// EJERCICIO 3: Usa shadowing para transformar un valor
// Convierte un texto en su longitud usando shadowing
fn ejercicio_3() {
    let mensaje = "Rust es genial";
    println!("Mensaje: {}", mensaje);

    let mensaje = mensaje.len();
    // Usa shadowing para redeclarar `mensaje` como su longitud (.len())
    // TODO: let mensaje = ...

    println!("Longitud del mensaje: {}", mensaje);
}

// EJERCICIO 4: Shadowing con cambio de tipo
// Primero es un número, luego es texto
fn ejercicio_4() {
    let valor = 42;
    println!("Valor numérico: {}", valor);
    let valor = "42";
    // Usa shadowing para convertir `valor` en un &str que diga "cuarenta y dos"
    // TODO: let valor = ...

    println!("Valor como texto: {}", valor);
}

fn main() {
    println!("\n=== EJERCICIO 1 ===");
    ejercicio_1();

    println!("\n=== EJERCICIO 2 ===");
    ejercicio_2();

    println!("\n=== EJERCICIO 3 ===");
    ejercicio_3();

    println!("\n=== EJERCICIO 4 ===");
    ejercicio_4();

    println!("\n✅ Todos los ejercicios completados!");
}
