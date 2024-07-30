use std::env;
use colored::Colorize;

pub fn coin_stdin()-> Vec<String> {
    // Captura los argumentos de la línea de comandos
    let args: Vec<String> = env::args().collect();

    // Asegúrate de que hay al menos un argumento (el nombre del programa)
    if args.len() < 2 {
      eprintln!("{}" , "------------------------------------------------------------  ".bright_red());
        eprintln!("{}{}" , "Por favor, proporciona nombre de token separados por".bright_red() , "(,)".bright_blue());
        eprintln!("{}" , "Ejemplo: bitcoin , polkadot , cardano ".bright_red());
        eprintln!("{}" , "---------------------------------------------------------- ".bright_red());
        std::process::exit(1);
    }

    // Ignora el primer argumento (el nombre del programa) y convierte el resto a números
    let mut coin: Vec<String> = Vec::new();
    for arg in &args[1..] {
        match arg.parse::<String>() {
            Ok(num) => coin.push(num),
            Err(_) => {
                eprintln!("Error: '{}' no es un número válido.", arg.bright_red());
                std::process::exit(1);
            }
        }
    }

    // Muestra el vector
   //println!("Vector cargado: {:?}", numeros);
   coin
}
