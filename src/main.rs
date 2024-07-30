//errore manejador
//use color_eyre::{eyre::Report,eyre::{eyre, WrapErr}, Section,};
//use tracing::{info, instrument};

mod ui;
use ui::draw;
use ui::render_token_details;


use std::io;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::layout::{Layout, Direction,Constraint}; // Add this line


mod api_call;
use tokio;
use colored::Colorize;
mod config;
mod coin_stdin;
mod vista;
use vista::print_coins_result01;
use api_call::{get_token_prices_by_times, Coin};
use api_call::plot_prices;



#[tokio::main]
//#[instrument]
 async fn main()->Result<(), Box<dyn std::error::Error>> {
/* 
{
    //imprimir datos y graficar 
    let token = "bitcoin";
    let days = 30;
   match get_token_prices_by_times(token, days).await {
        Ok(prices) => {
            plot_prices(prices);
        }
        Err(e) => {
            eprintln!("Error obteniendo  token prices: {}", e);
        }
    } 
 
}
 */

  //tomar datos del user  ---- vec!["polkadot", "bitcoin"];
 let vec_string =   coin_stdin::coin_stdin();
  // Convertir Vec<String> a Vec<&str>
  let coin: Vec<&str> = vec_string.iter().map(|s| s.as_str()).collect();

// let consulta = "all"; 
//levantar api_key de un archivo de configuracion 
 let (api_key01,  consulta) = config::leer_config();


 //process::exit(0);

//if consulta == "all"{ 
//  let result = api_call::get_coins_list_full(coin.clone(), &api_key01).await;
     /*  
      if result.is_ok(){
        //let dato = format!("{:#?}", result);
        //println!("{:#?}",result); 
        print_coins_result01(result.unwrap());
      }else{
        println!("{}" , "Error al obtener los datos. ".bright_red() );
      }
    */
//};
 
 // Run the UI using the correct module name
 // Configurar terminal
 enable_raw_mode()?;
 let mut stdout = io::stdout();
 execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
 let backend = CrosstermBackend::new(stdout);
 let mut terminal = Terminal::new(backend)?;

 // Ejecutar la aplicación
 let result = api_call::get_coins_list_full(coin.clone(), &api_key01).await;

 let res = run_app(&mut terminal, &result.unwrap());

 // Restaurar terminal
 //disable_raw_mode()?;
 //execute!(
 //    terminal.backend_mut(),
 //    LeaveAlternateScreen,
  //   DisableMouseCapture
 //)?;
 //terminal.show_cursor()?;

 if let Err(err) = res {
     println!("{:?}", err)
 }

 Ok(())
}


  fn run_app<B>(terminal: &mut Terminal<B>, initial_coins: &Vec<Coin>) -> io::Result<()> 
where
    B: ratatui::backend::Backend + std::io::Write, // Add this constraint
{
    
    let mut coins = initial_coins.clone();
    let mut selected_index = 0;
    let mut running = true; // Variable para controlar el bucle

    // Renderizar la interfaz completa inicialmente
    terminal.draw(|f| {
        ui::draw::<B>(f, &coins, selected_index); // Renderizar todos los tokens
        ui::draw::<B>(f, &coins, selected_index); // Renderizar detalles
        ui::draw::<B>(f, &coins, selected_index); // Renderizar gráfico
        ui::draw::<B>(f, &coins, selected_index); // Renderizar atajos
    })?;
// Habilitar captura de eventos
//enable_raw_mode()?;
//execute!(terminal.backend_mut(), EnterAlternateScreen, EnableMouseCapture)?;
coins = initial_coins.clone();
    while running {
        if let Ok(Event::Key(key)) = event::read() {
            match key.code {
                KeyCode::Char('q') => {
                    running = false; // Cambiar la variable a false para salir del bucle
                },
                KeyCode::Char('r') => {
                    coins = initial_coins.clone(); // Reiniciar la lista de monedas
                    terminal.draw(|f| {
                        ui::draw::<B>(f, &coins, selected_index); // Renderizar todos los tokens
                    })?;
                },
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                    terminal.draw(|f| {
                      ui::draw::<B>(f, &coins, selected_index); // Renderizar todos los tokens
                  })?;
                },
                KeyCode::Down => {
                    if selected_index < coins.len() - 1 {
                        selected_index += 1;
                    }
                    terminal.draw(|f| {
                      ui::draw::<B>(f, &coins, selected_index); // Renderizar todos los tokens
                  })?;
                },
                KeyCode::Char('d') => {
                    if let Some(selected_coin) = coins.get(selected_index) {
                        terminal.draw(|f| {
                            ui::draw::<B>(f, &coins, selected_index); // Renderizar detalles del token seleccionado
                        })?;
                    }
                },
                _ => {}
            }
        }
     


        std::thread::sleep(std::time::Duration::from_millis(50));
    }
// ... existing code ...
// Restaurar terminal:. Deshabilita el modo crudo y restaura la pantalla alterna.
disable_raw_mode()?; // Asegúrate de deshabilitar el modo crudo
execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
)?; // Restaurar la pantalla y deshabilitar la captura de mouse
terminal.show_cursor()?;
    Ok(()) // Retornar Ok al finalizar
}
 //--------------------fin ui--------------

 