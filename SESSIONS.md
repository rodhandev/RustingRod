# Registro de Sesiones de Estudio

Plantilla para cada sesión:

---
### Sesión XXX — YYYY-MM-DD
**Fase:** [número - nombre]
**Ejercicios trabajados:** [nombre/número]
**Conceptos cubiertos:** [lista]
**Errores encontrados y cómo los resolví:** [descripción]
**Notas / aprendizajes clave:** [texto libre]
---

## Sesiones

---
### Sesión 001 — 2026-06-17
**Tipo:** Diagnóstico inicial  
**Duración:** ~2 horas  
**Ejercicios trabajados:** diagnostico.rs (5/5 completados)

**Conceptos cubiertos:**
- Variables y shadowing vs mutabilidad
- Lifetimes explícitos en funciones con referencias
- Iteradores y métodos funcionales (.filter(), .collect())
- Enums personalizados para manejo de errores
- Structs con métodos (&self, &mut self)

**Errores encontrados y cómo los resolví:**
- Ejercicio 2: sintaxis `if s1.len() > s2.len() { s1 } else { s2 }` — necesitaba las llaves
- Ejercicio 4: confusión inicial sobre cómo definir variantes de enum con datos
- Ejercicio 5: puse `self` en `new()` (debía ser función asociada sin self), invertí tipo de retorno

**Aprendizajes clave:**
- Shadowing permite redeclarar variables sin `mut`
- Lifetimes conectan scope de referencias entre parámetros y retorno
- Iteradores son más idiomáticos que loops explícitos en Rust
- Enums pueden llevar datos asociados (Ok(i32), Err(String))
- Distinción clara entre funciones asociadas (new) y métodos (&self/&mut self)

**Estado al terminar:** Listo para rustlings fase 1
---

---
### Sesión 002 — 2026-06-18
**Tipo:** Práctica — variables y mutabilidad  
**Archivo trabajado:** tutor/01_variables.rs  
**Ejercicios:** 4/4 completados

**Conceptos cubiertos:**
- `let` (variables inmutables por defecto)
- `let mut` (variables mutables)
- Shadowing (redeclaración con `let`)
- Scope de variables (visibilidad en bloques)

**Errores encontrados y cómo los resolví:**
- E0384 (cannot assign twice to immutable variable): solucionado agregando `mut`
- E0425 (variable not found): variable declarada dentro de un bloque no visible fuera — moví declaración
- unused_mut warning: declaré `mut` pero nunca cambié el valor — removí `mut` innecesario

**Aprendizajes clave:**
- Rust detecta automáticamente `mut` innecesario con warnings
- Shadowing permite cambiar tipo de una variable, `mut` no
- Los mensajes del compilador de Rust son muy claros y ayudan a corregir errores
- Scope de variables es estricto: declarar dentro de `{}` limita visibilidad

**Estado al terminar:** let/mut/shadowing dominados, listo para tipos de datos
---

---
### Sesión 003 — 2026-06-19
**Tipo:** Teoría — representación binaria de números  
**Duración:** ~30 minutos  
**Ejercicios trabajados:** Conversión manual a IEEE 754 f32

**Conceptos cubiertos:**
- Binarización de enteros: división por 2, leer restos de abajo a arriba
- Binarización de fracciones: multiplicar por 2, tomar entero de arriba a abajo
- Forma normalizada IEEE 754: 1.XXX × 2ⁿ
- Cálculo de exponente con bias 127: n + 127, convertir a 8 bits
- Mantisa: bits después del "1." implícito, rellenar con ceros hasta 23 bits
- Por qué 0.1 decimal no tiene representación exacta en binario (loop infinito)

**Conversiones realizadas:**
- 5.0 → f32: ✅ correcto al segundo intento
- 2.0 → f32: ✅ correcto
- -4.0 → f32: ✅ correcto (signo negativo)
- 0.5 → f32: ✅ correcto (fracción)
- 3.5 → f32: ✅ correcto al primer intento

**Errores encontrados y cómo los resolví:**
- En 0.5: confundí parte entera con mantisa — puse "10000..." en vez de 23 ceros
- Entendí que la mantisa guarda solo lo que va DESPUÉS del "1." implícito

**Aprendizajes clave:**
- Enteros se binarizan con divisiones sucesivas por 2
- Fracciones se binarizan multiplicando por 2 y tomando el entero
- El "1." de la forma normalizada es implícito y NO se guarda en la mantisa
- Muchos decimales simples (como 0.1) no tienen representación exacta en binario
- Esto explica por qué f32/f64 tienen imprecisión en operaciones decimales

**Estado al terminar:** Comprensión sólida de IEEE 754, listo para continuar Fase 1 Rust
---

---
### Sesión 004 — 2026-06-19 (tarde)
**Tipo:** Práctica — tipos escalares en Rust  
**Duración:** ~40 minutos  
**Archivo trabajado:** tutor/02_tipos.rs (completado previamente)

**Conceptos cubiertos:**
- Tipos enteros: i8/i16/i32/i64, u8/u16/u32/u64
- Tipos flotantes: f32/f64
- Tipos primitivos: bool, char
- Comparación con C: int → i32, unsigned char → u8, double → f64
- Regla para elegir tipo: signo → i/u, rango → número de bits
- Casting con `as`: trunca bits menos significativos (≡ módulo 2ⁿ)
- Alternativas seguras para producción: `u8::try_from()`
- Mutabilidad vs shadowing: `mut` permite cambio, shadowing crea nueva variable

**Ejercicios resueltos:**
- Elegir tipos para edad/precio/flag → u8, f64, bool (refinó de i32 a u8)
- 300u32 as u8 → calculó 44 correctamente (truncamiento de bits)
- 1000u32 as u8 → calculó 232 correctamente: "solo sobreviven últimos 8 bits"
- ¿`let mut x` seguido de `let x` es mutable? → no, es nueva variable inmutable

**Aprendizajes clave:**
- `as` trunca bits — no satura ni hace panic (diferente a otros lenguajes)
- Shadowing ≠ mutabilidad: misma sintaxis del nombre pero variable nueva
- El `mut` NO se hereda en shadowing — la nueva variable es inmutable por defecto
- Casting en Rust es explícito siempre — evita bugs de conversión implícita

**Estado al terminar:** Tipos escalares dominados, listo para control de flujo
---
