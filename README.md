# Tarea 2
Esta es una tarea en la que se compara el rendimiento de 4 algoritmos para dibujar líneas rectas.

## Instrucciones de uso

Para utilizar el programa puede solo utilizar el comando `cargo run` seguido por los parámetros del programa. Alternativamente puede utilizar el comando `cargo build` y encontrará el ejecutable en el folder target/debug/tarea2, entonces podrá ejecutar el comando llamando el binario directamente seguido por los parámetros de uso. También se puede agregar el flag de `--release` a los comandos `cargo build --release` o  `cargo run --release` (el flag antes de los parámetros en el caso de `cargo run --release`) lo cual causará que el compilador no genere información de debuggeo, aunque de igual forma está configurado de forma que el compilador no haga optimizaciones.

Los parámetros del comando tienen la siguiente forma:
```
comando <resolución> <# líneas> <# veces>
```

Donde como se mencionó previamente `comando` puede corresponder a `cargo run` o a llamar el ejecutable directamente. La resolución es la resolución de la ventana cuadrada que se genera. El # de líneas es la cantidad de líneas a generar. El # de veces es la cantidad de veces que se repetirá el dibujo de todas las líneas para cada algoritmo. Todos los parámetros deben ser números enteros positivos y se ignorará cualquier argumento adicional.
