# Seguimiento de Progreso

## Semana actual

- **Fecha de inicio:** 2026-06-17
- **Fase activa:** Fase 1 - Fundamentos
- **Ejercicios rustlings completados:** 0 / 94
- **Ejercicios 100-exercises completados:** 0 / 100

## Metodología de aprendizaje (Proyecto Arandur)

**Filosofía:** Un concepto está aprendido solo cuando se ejecuta **perfecto Y rápido**, no cuando se toca superficialmente. Tocar un tema una vez no es suficiente — requiere repetición hasta automatización.

**Implicación práctica:** No avanzar ejercicios por avanzar. Cuando aparezcan errores repetidos en un concepto, generar más ejercicios del mismo tipo antes de continuar. El criterio de avance es: ejecución correcta + rápida, no solo "lo intentó".

**Para rustlings:** No saltar al siguiente tema hasta dominar el actual con precisión y velocidad.

## Diagnóstico inicial — 2026-06-17

- **Ejercicio 1 (variables/mutabilidad):** ✅ Resuelto — descubrió shadowing intuitivamente
- **Ejercicio 2 (lifetimes/&str):** ✅ Resuelto — lógica correcta con .len(), necesitó pista de sintaxis
- **Ejercicio 3 (iteradores/Vec):** ✅ Resuelto — escribió versión for loop y versión con iteradores
- **Ejercicio 4 (enums/Result):** ✅ Resuelto — necesitó explicación de enum, implementó correctamente
- **Ejercicio 5 (structs/métodos):** ✅ Resuelto — errores típicos de primer contacto (tipo invertido, self en new), corregidos rápido

### Conclusión del diagnóstico

- **Conceptos que ya maneja:** lógica de control, comparaciones, intuición de mutabilidad
- **Conceptos que necesitan práctica:** sintaxis Rust, tipos, self/&self/&mut self, enums
- **Fase recomendada:** empezar rustlings desde el ejercicio 1 (variables)
- **Observación:** resuelve bien cuando tiene la analogía en C como puente

## Registro de sesiones

Ver [SESSIONS.md](SESSIONS.md) para detalle completo de cada sesión.

## Errores frecuentes y aprendizajes

- **Shadowing vs mut:** En Rust se puede redeclarar variables con `let` (shadowing) sin necesidad de `mut`
- **Lifetimes:** Anotaciones explícitas `<'a>` indican que referencias tienen mismo scope
- **self en métodos:** `new()` no recibe self (función asociada), `get(&self)` para lectura, `incrementar(&mut self)` para modificación
- **unused_mut warning:** Declarar `mut` cuando el valor nunca cambia genera warning — Rust lo detecta automáticamente
- **E0425 (variable not found):** Variable declarada dentro de un bloque no es visible fuera de su scope
- **Mantisa en IEEE 754:** La mantisa guarda solo lo que va DESPUÉS del "1." implícito — el "1." nunca se almacena
- **0.1 en binario:** No tiene representación exacta (loop infinito) — por eso floats tienen imprecisión
- **Casting con `as`:** Trunca bits menos significativos (equivalente a módulo 2ⁿ) — no satura ni hace panic. Para producción usar `try_from()`
- **Shadowing no hereda `mut`:** `let mut x` seguido de `let x` crea nueva variable inmutable
- **Acumuladores en loops:** Declarar dentro del loop los reinicia cada iteración — declarar fuera del loop
- **break sin valor:** `break;` retorna `()`, `break valor;` retorna el valor
- **Rangos inclusivos vs exclusivos:** `0..5` excluye el 5, `0..=5` incluye el 5
- **Mezclar input con acumulador:** No usar la variable de entrada como acumulador — se pisan mutuamente
- **Métodos sobre tipos:** `.to_string()`, `.parse()`, `.len()` — aún necesita consolidar mecánica de métodos
