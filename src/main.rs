use std::io;

fn suma(numero1: i128, numero2: i128) -> i128 {
    numero1 + numero2
}

fn resta(numero1: i128, numero2: i128) -> i128 {
    numero1 - numero2
}

fn multiplicacion(numero1: i128, numero2: i128) -> i128 {
    numero1 * numero2
}

fn division(numero1: i128, numero2: i128) -> i128 {
    numero1 / numero2
}

fn potencia(numero1: i128, numero2: i128) -> i128 {
    numero1 ** numero2
}
fn operacion(accion: &str, numero1: i128, numero2: i128) -> i128 {
    match accion {
        "suma" => suma(numero1, numero2),
        "resta" => resta(numero1, numero2),
        "multiplicacion" => multiplicacion(numero1, numero2),
        "potencia" => potencia(numero1, numero2),
        "division" => {
            if numero2 != 0 {
                division(numero1, numero2)
            } else {
                println!("Error: No se puede dividir por cero");
                0
            }
        }
        _ => {
            println!("Acción no válida");
            0 // Valor por defecto en caso de una acción no válida
        }
    }
}

fn main() {
    let mut input = String::new();

    // Leer la operación desde la consola
    println!("Introduce la operación (suma, resta, multiplicacion, potencia, division): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");
    let accion = input.trim().to_string(); // Copiar el resultado de trim a un String nuevo

    // Leer el primer número desde la consola
    input.clear(); // Ahora es seguro mutar `input`
    println!("Introduce el primer número: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");

    let numero1: i128 = input
        .trim()
        .parse()
        .expect("Por favor, ingresa un número válido");

    // Leer el segundo número desde la consola
    input.clear();
    println!("Introduce el segundo número: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");

    let numero2: i128 = input
        .trim()
        .parse()
        .expect("Por favor, ingresa un número válido");

    // Realizar la operación
    let resultado = operacion(&accion, numero1, numero2); // Usamos `&accion` porque `operacion` espera un `&str`
    println!("El resultado es: {}", resultado);
}
