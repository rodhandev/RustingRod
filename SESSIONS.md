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
