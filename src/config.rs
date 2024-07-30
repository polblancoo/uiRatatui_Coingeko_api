use dotenv::dotenv;
use std::env;

pub fn leer_config()->(String , String){
 // Leyendo el dotenv, si el archivo
    // .env existe, entonces lo utilizara como
    // variables de entorno, si no existe,
    // entonces intentara obtener las variables de entorno
    // desde el sistema operativo directamente
    dotenv().ok();

    // Ahora podemos leer las variables que se establecieron en el archivo .env como si fueran
    // variables de entorno, recuerda que si no fueron definidos en un .env, entonces intentara
    // obtener los valores de las variables de entorno en el Sistema Operativo.
    let apikey: String = env::var("APIKEY").expect("No se encuentra la variable de entorno ( API_KEY de Coingecko)");
    let consulta: String = env::var("CONSULTA").expect("No se encuentra la variable de entorno (tipo consulta )");
    //println!("Valor de la variable de entorno APIKEY = {}", APIKEY);
  (apikey, consulta)

}