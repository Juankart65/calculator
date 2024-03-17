use std::io::{self, Write};

#[derive(Debug, PartialEq)]
enum Operacion {
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Salir,
}

fn realizar_operacion(operacion: Operacion, num1: f64, num2: f64) -> f64 {
    match operacion {
        Operacion::Suma => num1 + num2,
        Operacion::Resta => num1 - num2,
        Operacion::Multiplicacion => num1 * num2,
        Operacion::Division => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("¡Error! No se puede dividir por cero.");
                std::f64::NAN
            }
        }
        Operacion::Salir => {
            println!("Saliendo del programa. ¡Hasta luego!");
            std::process::exit(0);
        }
    }
}

fn main() {
    loop {
        println!("--- Calculadora ---");
        println!("1. Suma");
        println!("2. Resta");
        println!("3. Multiplicación");
        println!("4. División");
        println!("5. Salir");

        print!("Seleccione una operación (1-5): ");
        io::stdout().flush().unwrap();

        let mut operacion_str = String::new();
        io::stdin().read_line(&mut operacion_str).unwrap();
        let operacion: Operacion = match operacion_str.trim().parse() {
            Ok(op) => match op {
                1 => Operacion::Suma,
                2 => Operacion::Resta,
                3 => Operacion::Multiplicacion,
                4 => Operacion::Division,
                5 => Operacion::Salir,
                _ => {
                    println!("¡Error! Seleccione una opción válida.");
                    continue;
                }
            },
            Err(_) => {
                println!("¡Error! Ingrese un número válido.");
                continue;
            }
        };

        if operacion == Operacion::Salir {
            break;
        }

        print!("Ingrese el primer número: ");
        io::stdout().flush().unwrap();
        let num1: f64 = match leer_numero() {
            Some(num) => num,
            None => continue,
        };

        print!("Ingrese el segundo número: ");
        io::stdout().flush().unwrap();
        let num2: f64 = match leer_numero() {
            Some(num) => num,
            None => continue,
        };

        let resultado = realizar_operacion(operacion, num1, num2);
        println!("Resultado: {}\n", resultado);
    }
}

fn leer_numero() -> Option<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("¡Error! Ingrese un número válido.");
            None
        }
    }
}
