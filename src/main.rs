//mod constants;
mod lines;

//use constants::*;
use lines::{Line, Lines};
use rand::distributions::{Distribution, Uniform};
use sdl_wrapper::ScreenContextManager;
use std::{
    env, process,
    thread::sleep,
    time::{Duration, Instant},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parseando argumentos: {}", err);
        process::exit(1);
    });

    let mut rng = rand::thread_rng();
    let distribution = Uniform::new(0, config.resolucion);

    // Generar vector de líneas
    let lines: Vec<Line> = (0..config.lineas)
        .map(|_| Line {
            x0: distribution.sample(&mut rng),
            y0: distribution.sample(&mut rng),
            x1: distribution.sample(&mut rng),
            y1: distribution.sample(&mut rng),
        })
        .collect();

    // Inicializar context manager de la ventana
    let mut screen = ScreenContextManager::new("Tarea1", config.resolucion, config.resolucion);

    screen.set_color(0.86, 0.46, 0.52);

    // Closure que dibuja líneas y hace benchmark
    let mut benchmark = |funct: fn(&mut ScreenContextManager, &Line), name: &str| {
        // Limpiar pantalla
        screen.clear(0.0);
        screen.present();

        // Benchmark
        let start = Instant::now();
        for _ in 0..config.veces {
            for i in 0..config.lineas as usize {
                //println!("Dibujando línea: {:?}", &lines[i]);
                funct(&mut screen, &lines[i]);
            }
        }

        // Obtener y presentar resultados
        let time = start.elapsed();
        screen.present();
        println!(
            "\nTiempo que tardó la función {}:\
            \n--------------------------\n\
            Nanosegundos: {}\nSegundos: {}\
            \n--------------------------",
            name,
            time.as_nanos(),
            time.as_secs_f32()
        );
    };

    // Probar diferentes funciones
    benchmark(ScreenContextManager::naive_line, "naive approach");
    sleep(Duration::from_secs(1));
    benchmark(ScreenContextManager::incremental_line, "incremental");
    sleep(Duration::from_secs(1));
    benchmark(ScreenContextManager::better_line, "mejorada");
    sleep(Duration::from_secs(1));
    benchmark(ScreenContextManager::bresenham_line, "bresenham");
    sleep(Duration::from_secs(1));
}

struct Config {
    resolucion: u32,
    lineas: u32,
    veces: u32,
}

fn parse_args(args: &[String]) -> Result<Config, &str> {
    if args.len() < 4 {
        return Err("Faltan argumentos. El comando se llama de la forma ./tarea2 <resolución> <# líneas> <# veces>");
    }

    let resolucion = match args[1].parse() {
        Ok(v) => v,
        Err(_e) =>  return Err("El argumento #1 no se pudo convertir a entero. Todos los argumentos deben ser números enteros."),};
    let lineas = match args[2].parse() {
        Ok(v) => v,
        Err(_e) =>  return Err("El argumento #2 no se pudo convertir a entero. Todos los argumentos deben ser números enteros."),};
    let veces = match args[3].parse() {
        Ok(v) => v,
        Err(_e) =>  return Err("El argumento #3 no se pudo convertir a entero. Todos los argumentos deben ser números enteros."),};
    Ok(Config {
        resolucion,
        lineas,
        veces,
    })
}
