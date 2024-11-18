PROYECTO 3
# Sistema Solar - Simulador Gráfico

Enlace video: https://www.loom.com/share/ffa063330a6f415287562ec6aae8f209?sid=b5efe5f1-114b-46d3-be4e-b2b1d785f9a0

Este proyecto es un simulador gráfico que representa un sistema solar simplificado, con una nave espacial modelada en 3D que puedes mover por la escena. El sistema está renderizado utilizando Rust y la librería **minifb**. Este simulador incluye varios cuerpos celestes (planetas, estrellas, luna, anillo) y sus respectivas órbitas, así como un skybox que simula el espacio exterior.

## Características del Proyecto

- **Nave 3D Modelada**: Una nave espacial representada en la escena, que se mueve junto con la cámara.
- **Cuerpos Celestes**: El sistema solar tiene una estrella emisiva, un planeta rocoso, un gigante gaseoso, un anillo y una luna.
- **Órbitas Visuales**: Cada cuerpo celeste tiene una órbita visible dibujada alrededor de la estrella central.
- **Skybox con Estrellas**: El skybox muestra un fondo estrellado, creando la sensación de estar en el espacio.
- **Control de Cámara**: Puedes mover la cámara para explorar la escena, con las teclas WASD.
- **Instant Warping Animado**: Presiona la tecla `I` para hacer warping instantáneo a un punto diferente del sistema solar, y `O` para regresar a la posición inicial.

## Controles

- **WASD**: Mueve la cámara en la dirección deseada.
- **Esc**: Cierra la aplicación.
- **I**: Realiza un warping instantáneo a un punto aleatorio del sistema solar.
- **O**: Regresa a la posición inicial después del warping.

## Requisitos del Sistema

- **Rust** (1.50 o superior)
- **Cargo** (Para compilar y correr el proyecto)
- **Librería minifb**: Asegúrate de tener instalada la librería **minifb** para Rust.

## Instrucciones de Instalación

1. **Clona el Repositorio**
   ```sh
   git clone <URL_DEL_REPOSITORIO>
   cd sistema_solar
   ```

2. **Compila el Proyecto**
   ```sh
   cargo build
   ```

3. **Ejecuta el Proyecto**
   ```sh
   cargo run
   ```

## Estructura del Proyecto

- **main.rs**: Archivo principal donde se inicializa la ventana, la cámara y se manejan los controles.
- **camera.rs**: Módulo que maneja el movimiento y la lógica de la cámara.
- **spaceship.rs**: Define la estructura y comportamiento de la nave espacial.
- **render.rs**: Contiene las funciones de renderizado de la nave, cuerpos celestes, skybox y órbitas.
- **shaders.rs**: Define los shaders utilizados para los diferentes objetos del sistema (estrella emisiva, planeta rocoso, gigante gaseoso, anillo y luna).

## Descripción de las Características

### 1. **Skybox con Estrellas**
El fondo del simulador tiene un skybox lleno de estrellas que le da un toque realista al espacio exterior. Las estrellas están distribuidas aleatoriamente y de forma sutil para evitar distracciones visuales.

### 2. **Órbitas Visibles**
Cada planeta tiene una órbita bien definida para mostrar el recorrido alrededor de la estrella central. Las órbitas son círculos blancos que resaltan sobre el fondo oscuro.

### 3. **Warping Instantáneo**
Una de las características más destacadas es la posibilidad de "teletransportarse" (warping) a un punto aleatorio del sistema solar, lo que permite una exploración rápida de la escena.

### 4. **Modelado 3D de la Nave Espacial**
La nave espacial está modelada en 3D y representada por puntos verdes. El usuario puede explorar la escena moviendo la cámara para ver la nave desde diferentes ángulos.

## Mejoras Potenciales
- **Animación de Planetas**: Actualmente los planetas están estáticos. Podría agregarse una animación para que se desplacen a lo largo de sus órbitas.
- **Mayor Interacción**: Agregar controles adicionales para rotar los planetas o incluso cambiar la velocidad de la cámara.

## Problemas Conocidos
- **Performance**: En algunas máquinas, el renderizado puede ser lento, especialmente si no se está ejecutando en modo de producción. Ejecuta el proyecto en modo "release" para mejorar la performance:
  ```sh
  cargo run --release
  ```

## Licencia
Este proyecto está bajo la licencia MIT. Sientete libre de usarlo, modificarlo y distribuirlo.

![image](https://github.com/user-attachments/assets/7784209e-89ce-483f-bf94-59a481877a0e)
![image](https://github.com/user-attachments/assets/b4fcdfc0-ebe8-4cfd-a5c7-607b38853ea0)
![image](https://github.com/user-attachments/assets/d2d55a8d-6f20-4a12-8858-85d48663206e)
![image](https://github.com/user-attachments/assets/a05ecb06-36ca-45ad-a9bc-7576e3400496)
![image](https://github.com/user-attachments/assets/625daa43-5192-4a29-b431-6ce2fae88d5a)
