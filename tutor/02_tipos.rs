// ============================================================
// CONCEPTO: Tipos de datos básicos en Rust
// ============================================================
// Rust es un lenguaje con tipado estático fuerte. El compilador
// puede inferir tipos en muchos casos, pero también puedes
// especificarlos explícitamente.
//
// TIPOS ENTEROS:
//   Signed:   i8, i16, i32, i64, i128
//   Unsigned: u8, u16, u32, u64, u128
//   Por defecto: i32
//
// TIPOS FLOTANTES:
//   f32, f64
//   Por defecto: f64
//
// BOOLEANO:
//   bool → true o false
//
// CARÁCTER:
//   char → usa comillas simples 'a' (no dobles "a")
//   Representa un Unicode Scalar Value (4 bytes)
//
// INFERENCIA vs ANOTACIÓN:
//   let x = 5;         // Rust infiere i32
//   let x: i64 = 5;    // Anotación explícita
//
// CONVERSIÓN DE TIPOS (casting):
//   let x: i32 = 10;
//   let y: f64 = x as f64;  // conversión explícita con `as`
//
// ANALOGÍA C:
//   int           ≈  i32
//   unsigned int  ≈  u32
//   float         ≈  f32
//   double        ≈  f64
//   char          ≈  char (pero char de Rust es Unicode, 4 bytes)
//   bool          ≈  _Bool (C99) o int usado como bool
//
// DIFERENCIA CLAVE: En C, char es 1 byte ASCII. En Rust,
// char es 4 bytes Unicode. Para bytes usa u8.
// ============================================================

// EJERCICIO 1: Declara variables de cada tipo básico
// Usa anotación explícita de tipo para cada una
fn ejercicio_1() {
    // TODO: Declara las siguientes variables con anotación explícita:
    // - edad: u8 = 25
    let edad: u8 = 25;
    // - temperatura: f32 = 36.6
    let temperatura: f32 = 36.6;
    // - es_estudiante: bool = true
    let es_estudiante: bool = true;
    // - inicial: char = 'R'
    let inicial: char = 'R';
    // - poblacion: u64 = 8000000
    let poblacion: u64 = 8000000;

    println!("Edad: {} años", edad);
    println!("Temperatura: {}°C", temperatura);
    println!("¿Es estudiante?: {}", es_estudiante);
    println!("Inicial del nombre: {}", inicial);
    println!("Población: {} habitantes", poblacion);
}

// EJERCICIO 2: Inferencia de tipos
// Deja que Rust infiera los tipos automáticamente
fn ejercicio_2() {
    // Declara estas variables SIN anotación de tipo
    // Rust inferirá el tipo basándose en el uso

    // TODO: let x = 10;
    let x = 10;
    // TODO: let y = 20;
    let y = 20;
    
    let suma = x + y;
    println!("La suma de {} + {} = {}", x, y, suma);

    // TODO: let pi = 3.1415
    let pi = 3.1415;
    // TODO: let radio = 5.0;
    let radio = 5.0;

    let area = pi * radio * radio;
    println!("Área del círculo con radio {}: {}", radio, area);
}

// EJERCICIO 3: Arregla los errores de tipo
// Este código tiene errores de tipo, corrígelos
fn ejercicio_3() {
    // ERROR: no puedes asignar un float a un i32
    let x: f32 = 3.14;
    println!("x = {}", x);

    // ERROR: char usa comillas simples, no dobles
    let letra: char = 'a';
    println!("letra = {}", letra);

    // ERROR: bool es true/false, no 0/1
    let activo: bool = true;
    println!("activo = {}", activo);
}

// EJERCICIO 4: Conversión de tipos con `as`
// Convierte tipos para hacer división con decimales
fn ejercicio_4() {
    let a: i32 = 7;
    
    let b: i32 = 2;

    // División entera (trunca decimales)
    let division_entera = a / b;
    println!("División entera: {} / {} = {}", a, b, division_entera);

    // TODO: Convierte a y b a f64 usando `as` para obtener 3.5
    // Pista: let resultado = (a as f64) / (b as f64);
     let division_decimal = (a as f64) / (b as f64);
    // let division_decimal = ...

    println!("División decimal: {} / {} = {}", a, b, division_decimal);
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
