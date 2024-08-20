struct Datos {
    esta_vivo: bool,
    nombre: String,
    edad: i8,
}

fn mayor_de_edad(edad: i8, nombre: String) -> Result<Datos, String> {
    if edad < 18 {
        Err(format!("Eres menor de edad, {}. No puedes ingresar.", nombre))
    } else if edad >= 128 {
        Err(format!("Edad no aceptada, {}. Edad fuera de rango.", nombre))
    } else {
        println!("Edad aceptada");
        Ok(Datos {
            esta_vivo: true,
            nombre,
            edad,
        })
    }
}

fn validar_nombre(nombre: String, edad: i8) -> Result<Datos, String> {
    if nombre.trim().is_empty() {
        Err(String::from("El nombre no puede estar vacío"))
    } else {
        mayor_de_edad(edad, nombre)
    }
}

fn main() {
    match validar_nombre(String::from("Juan"), 2120) {
        Ok(datos) => println!(
            "Nombre: {}, Edad: {}, ¿Está vivo?: {}",
            datos.nombre, datos.edad, datos.esta_vivo
        ),
        Err(e) => println!("Error: {}", e),
    }
}
