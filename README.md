# Técnicas de Programación Concurrente I - 1C 2023

En el repositorio se encuentran resueltos algunos problemas clásicos de la concurrencia que se ven en la materia, utilizando distintas herramientas de sincronismo así como distintos modelos de concurrencia (actores, programación asincrónica, modelo fork-join, estado mutable compartido). También hay algunos ejercicios prácticos resueltos de finales anteriores.

## Final 1ra fecha

Me presenté en la primera fecha del final de la materia y, dado que nos presentamos 3 alumnos en total, el final consistió de una parte práctica (escrito) y de una parte teórica (oral).

### Parte práctica

Consistía de 2 ejercicios prácticos para hacer en 45 minutos:  

1. Implementar un servidor TCP que responde a peticiones con programación asincrónica en Rust.
2. Realizar un diagrama de tiempo de las funciones invocadas en el servidor y mostrar en qué puntos del programa el servidor se bloquearía en una versión sincrónica.

### Parte teórica

- Propiedades para garantizar el criterio de corrección: safety y liveness.
- Problema de la sección crítica.
- Semáforos.
- Monitores.
- Redes de petri: problema del productor y consumidor con buffer acotado.
