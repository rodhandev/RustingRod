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
