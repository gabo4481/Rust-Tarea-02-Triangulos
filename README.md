# Cálculo de Área de Triángulos en Rust

Este proyecto en Rust calcula el área de un número determinado de triángulos a partir de las coordenadas de sus vértices. El programa permite al usuario ingresar las coordenadas de los vértices de cada triángulo, calcula el área utilizando la fórmula de Herón, y guarda los resultados en un archivo de texto.

## Requisitos

- Rust 1.XX o superior.
  
## Descripción

El programa realiza los siguientes pasos:

1. Solicita al usuario la cantidad de triángulos que desea calcular.
2. Para cada triángulo, solicita las coordenadas de sus tres vértices.
3. Calcula la distancia entre los vértices y utiliza la fórmula de Herón para calcular el área del triángulo.
4. Guarda las coordenadas de los vértices y el área de cada triángulo en un archivo `triangulos.txt`.

## Funcionalidades

- **Cálculo de la distancia**: Utiliza la fórmula de distancia entre dos puntos en el plano cartesiano.
- **Cálculo del área de un triángulo**: Se usa la fórmula de Herón para calcular el área a partir de las distancias entre los vértices.
- **Guardar resultados**: Los resultados se guardan en un archivo de texto, con las coordenadas de los tres vértices y el área de cada triángulo.

## Instrucciones de uso

1. Clona este repositorio o copia el código a tu entorno de desarrollo.
2. Asegúrate de tener Rust instalado en tu máquina. Si no lo tienes, instálalo desde [https://www.rust-lang.org/](https://www.rust-lang.org/).
3. Compila y ejecuta el programa con el siguiente comando:

   ```bash
   cargo run
