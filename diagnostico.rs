// RustingRod — Diagnóstico Inicial
// Sesión 001 — 2026-06-17
//
// Este archivo contiene 5 ejercicios para evaluar tu nivel actual en Rust.
// NO busques ayuda externa. Si te trabas, déjalo y continúa con el siguiente.
//
// Instrucciones:
// - Escribe tu solución para cada ejercicio
// - Intenta compilar con: rustc diagnostico.rs
// - Anota los errores que aparezcan
// - Pasa al siguiente ejercicio

// ============================================================================
// EJERCICIO 1 — Variables y mutabilidad
// ============================================================================
// TODO: Arregla el código para que compile sin warnings

fn ejercicio_1() {
    let x = 5;
    println!("El valor de x es: {}", x);
    let x = 6;
    println!("El valor de x ahora es: {}", x);
}

fn main() {
    ejercicio_1();
    ejercicio_2();
    ejercicio_3();
    ejercicio_4();
    ejercicio_5();
}
// ============================================================================
// ESPERA — No sigas hasta completar el ejercicio 1
// ============================================================================

// ============================================================================
// EJERCICIO 2 — Lifetimes
// ============================================================================
// TODO: Implementa una función que tome dos &str y devuelva el más largo
// Debes usar lifetimes explícitos

fn mas_largo<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
    s1
} else {
    s2
}// Tu código aquí
}

fn ejercicio_2() {
    let texto1 = "hola";
    let texto2 = "mundo";
    let resultado = mas_largo(texto1, texto2);
    println!("El más largo es: {}", resultado);
}

// ============================================================================
// ESPERA — No sigas hasta completar el ejercicio 2
// ============================================================================

// ============================================================================
// EJERCICIO 3 — Iteradores
// ============================================================================
// TODO: Filtra los números pares de un Vec<i32> y devuelve un Vec nuevo
// Usa métodos de iteradores (.iter(), .filter(), .collect())

fn filtrar_pares(numeros: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::new();
    for n in numeros {
        if n % 2 == 0 {
            resultado.push(n);
        }
    }
    resultado // Tu código aquí
 }

fn filtrar_pares2(numeros: Vec<i32>) -> Vec<i32> {
    numeros.into_iter().filter(|&n| n % 2 == 0).collect()
}

fn ejercicio_3() {
     let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
     let pares = filtrar_pares2(nums);
     println!("Números pares: {:?}", pares);
}
 

// ============================================================================
// ESPERA — No sigas hasta completar el ejercicio 3
// ============================================================================

// ============================================================================
// EJERCICIO 4 — Enums y manejo de errores
// ============================================================================
// TODO: Define un enum MiResultado con variantes Ok(i32) y Err(String)
// Implementa una función dividir que devuelva error si el divisor es 0

 enum MiResultado {
     Ok(i32),
     Err(String),
 }

 fn dividir(a: i32, b: i32) -> MiResultado {
     if b == 0 {
         MiResultado::Err(String::from("division by cero"))
     } else {
         MiResultado::Ok(a / b)
     }
 }

 fn ejercicio_4() {
     match dividir(10, 2) {
         MiResultado::Ok(resultado) => println!("10 / 2 = {}", resultado),
         MiResultado::Err(msg) => println!("Error: {}", msg),
     }
//
     match dividir(10, 0) {
         MiResultado::Ok(resultado) => println!("10 / 0 = {}", resultado),
         MiResultado::Err(msg) => println!("Error: {}", msg),
     }
 }

// ============================================================================
// ESPERA — No sigas hasta completar el ejercicio 4
// ============================================================================

// ============================================================================
// EJERCICIO 5 — Structs y métodos
// ============================================================================
// TODO: Define un struct Contador con campo valor: i32
// Implementa método get(&self) que devuelva el valor
// Implementa método incrementar(&mut self) que sume 1 al valor

 struct Contador {
     valor: i32,// Tu código aquí
 }

 impl Contador {
     
     fn incrementar(&mut self) {
        self.valor += 1;
     }
     fn new() -> Contador {
        Contador { valor: 0}
     }
     fn get(&self) -> i32 {
        self.valor
     }
//
//     // Implementa get() e incrementar() aquí
 }

 fn ejercicio_5() {
     let mut contador = Contador::new();
     println!("Valor inicial: {}", contador.get());
     contador.incrementar();
     contador.incrementar();
     println!("Valor después de incrementar 2 veces: {}", contador.get());
 }

// ============================================================================
// FIN DEL DIAGNÓSTICO
// ============================================================================
