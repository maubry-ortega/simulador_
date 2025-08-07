# 🧬 Simulador Evolutivo

Un simulador de vida artificial y evolución desarrollado en Rust usando el motor de juegos Bevy. Este proyecto simula un ecosistema con criaturas herbívoras, depredadores y plantas, donde la evolución ocurre a través de reproducción con mutaciones genéticas.

## 🎯 Características

### 🦎 Criaturas Herbívoras
- **Genética**: Velocidad, tamaño y color heredables con mutaciones
- **Estados de comportamiento**: Vagando, buscando comida, reproduciéndose
- **Supervivencia**: Deben comer plantas para mantener energía
- **Evolución**: Las características se transmiten y mutan entre generaciones

### 🦊 Depredadores
- **Caza activa**: Buscan y devoran criaturas herbívoras
- **Reproducción estacional**: Se reproducen cuando tienen suficiente energía
- **Comportamiento adaptativo**: Cambian entre caza y reproducción

### 🌿 Plantas
- **Crecimiento orgánico**: Aumentan de tamaño con el tiempo
- **Reproducción**: Se multiplican automáticamente cada cierto tiempo
- **Fuente de energía**: Alimento principal para herbívoros

### 🧠 Sistemas de IA
- **Búsqueda de comida**: Las criaturas hambrientas buscan plantas cercanas
- **Evasión de depredadores**: Huyen cuando detectan amenazas
- **Separación**: Evitan amontonarse entre sí
- **Límites del mundo**: Rebotan en los bordes de la pantalla

## 🚀 Instalación y Ejecución

### Prerrequisitos
- Rust 1.70+ instalado
- Cargo (incluido con Rust)

### Ejecutar el simulador
```bash
git clone <repositorio>
cd simulador_
cargo run --release
```

## 🎮 Controles

El simulador actualmente se ejecuta automáticamente. Observa:
- **Círculos verdes pequeños**: Plantas
- **Círculos verdes medianos**: Criaturas herbívoras
- **Círculos rojos grandes**: Depredadores

## 📊 Estadísticas en Pantalla

El HUD muestra información en tiempo real:
- 🧬 **Criaturas**: Número actual de herbívoros
- 🦊 **Depredadores**: Número actual de depredadores  
- 🌿 **Plantas**: Número actual de plantas
- 🔁 **Reproducciones**: Total de reproducciones ocurridas
- 💀 **Muertes**: Total de muertes registradas
- 📈 **Máx Gen**: Generación más alta alcanzada
- 📊 **Prom Gen**: Generación promedio actual
- ⏱️ **Tiempo**: Tiempo de simulación transcurrido
- **FPS**: Cuadros por segundo

## 🧬 Sistema de Evolución

### Genética
Cada criatura tiene genes que determinan:
- **Velocidad**: Qué tan rápido se mueve (10-100)
- **Tamaño**: Tamaño visual de la criatura (5-50)
- **Color**: Color HSL con mutaciones

### Mutaciones
Durante la reproducción ocurren mutaciones aleatorias:
- **Velocidad**: ±5 unidades (con límites)
- **Tamaño**: ±2 unidades (con límites)
- **Color**: Nuevo color HSL aleatorio

### Selección Natural
- Las criaturas deben sobrevivir para reproducirse
- Necesitan energía suficiente (>120) y tiempo desde última reproducción (>5s)
- Los depredadores controlan la población de herbívoros
- Las plantas limitan la capacidad de carga del ecosistema

## 🏗️ Arquitectura del Código

### Estructura de Directorios
```
src/
├── components.rs      # Componentes ECS (Organism, Genes, etc.)
├── resources.rs       # Recursos globales (Stats)
├── main.rs           # Configuración principal de Bevy
├── utils.rs          # Utilidades (mutaciones, colores)
├── systems/          # Sistemas de juego
│   ├── setup.rs      # Inicialización del mundo
│   ├── movement.rs   # Movimiento y física básica
│   ├── states.rs     # Estados de comportamiento IA
│   ├── plant.rs      # Lógica de plantas
│   ├── predator.rs   # Lógica de depredadores
│   ├── reproduction.rs # Sistema de reproducción
│   ├── boundaries.rs # Límites del mundo
│   ├── collisions.rs # Evitar superposiciones
│   └── hud.rs        # Interfaz de usuario
└── utils/
    └── factory.rs    # Funciones de creación de entidades
```

### Componentes Principales
- `Organism`: Energía, edad, generación
- `Creature`: Datos específicos de herbívoros
- `Predator`: Datos específicos de depredadores
- `Genes`: Información genética heredable
- `Plant`: Datos de plantas
- `State`: Estados de comportamiento (Wandering, SeekingFood, etc.)

## 🔧 Configuración

### Parámetros de Simulación
Los valores clave se pueden ajustar en el código:

```rust
// Población inicial (setup.rs)
spawn_initial_creatures: 10 criaturas
spawn_initial_predators: 2 depredadores  
spawn_initial_plants: 30 plantas

// Reproducción (reproduction.rs)
energia_minima_reproduccion: 120.0
tiempo_entre_reproducciones: 5.0 segundos
costo_energetico_reproduccion: 40.0

// Supervivencia (movement.rs)
consumo_energia_criaturas: 1.0/segundo
consumo_energia_depredadores: 0.8/segundo
edad_maxima_criaturas: 60.0 segundos
```

## 🎓 Conceptos Demostrados

Este simulador ilustra conceptos fundamentales de:

- **Algoritmos Genéticos**: Selección, cruzamiento, mutación
- **Vida Artificial**: Comportamientos emergentes, ecosistemas
- **Sistemas Multi-Agente**: Interacciones entre diferentes tipos de entidades
- **Programación de Juegos**: ECS (Entity Component System) con Bevy
- **Simulación**: Modelado de sistemas complejos

## 🚧 Limitaciones Conocidas

- No hay persistencia de datos entre ejecuciones
- Parámetros hardcodeados (no ajustables en runtime)
- Genética simple (solo 3 rasgos)
- Sin controles de usuario para pausar/reiniciar
- Sin gráficos de estadísticas históricas

## 🔮 Mejoras Futuras

### Características Planeadas
- [ ] **Controles de usuario**: Pause, reset, ajustar velocidad
- [ ] **Más rasgos genéticos**: Resistencia, visión, metabolismo
- [ ] **Gráficos de estadísticas**: Historial de población, diversidad genética
- [ ] **Configuración externa**: Archivo de configuración JSON/TOML
- [ ] **Guardado de simulaciones**: Exportar/importar estados
- [ ] **Diferentes tipos de plantas**: Con valores nutricionales variados
- [ ] **Terreno**: Obstáculos, diferentes biomas
- [ ] **Más tipos de criaturas**: Omnívoros, especies especializadas

### Mejoras Técnicas
- [ ] **Optimización de rendimiento**: Spatial hashing para colisiones
- [ ] **UI mejorada**: Panel de control, visualización de genes
- [ ] **Herramientas de análisis**: Exportar datos para análisis científico
- [ ] **Multithreading**: Sistemas paralelos para mejor rendimiento

## 📚 Recursos de Aprendizaje

- [Bevy Engine](https://bevyengine.org/) - Motor de juegos usado
- [Rust Programming Language](https://www.rust-lang.org/) - Lenguaje de programación
- [Entity Component System](https://en.wikipedia.org/wiki/Entity_component_system) - Patrón arquitectónico
- [Genetic Algorithms](https://en.wikipedia.org/wiki/Genetic_algorithm) - Algoritmos evolutivos

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Por favor:

1. Fork el repositorio
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para detalles.

## 👨‍💻 Autor

Desarrollado como proyecto educativo para demostrar conceptos de vida artificial y algoritmos evolutivos usando Rust y Bevy.

---

*¡Observa como la vida artificial evoluciona ante tus ojos! 🧬✨*