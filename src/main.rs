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
 disable_raw_mode()?;
 execute!(
     terminal.backend_mut(),
     LeaveAlternateScreen,
     DisableMouseCapture
 )?;
 terminal.show_cursor()?;

 if let Err(err) = res {
     println!("{:?}", err)
 }

 Ok(())
}

  fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, initial_coins: &Vec<Coin>) -> io::Result<()> {
 
  let mut coins = Vec::new();
  coins = initial_coins.clone();
 
  let mut selected_index = 0;

  
  loop {
    terminal.draw(|f: &mut ratatui::Frame| {
        ui::draw::<B>(f, &coins, selected_index);

        // Mueve la lógica de eventos aquí para tener acceso a 'f'
        if let Ok(Event::Key(key)) = event::read() {
            match key.code {
                KeyCode::Char('q') => return (),
                KeyCode::Char('r') => {
                    // Actualizar los datos cuando se presiona 'r'
                    coins = initial_coins.clone();
                },
                KeyCode::Up => {
                    // Mover selección hacia arriba
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                },
                KeyCode::Down => {
                    // Mover selección hacia abajo
                    if selected_index < coins.len() - 1 {
                        selected_index += 1;
                    }
                },
                KeyCode::Char('d') => {
                    // Llamar a render_token_details al presionar 'd'
                    if let Some(selected_coin) = coins.get(selected_index) {
                        let right_chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                            .split(f.size());

                        render_token_details::<B>(f, selected_coin, right_chunks[1]); // Ahora 'f' es accesible
                    }
                },
                _ => {}
            }
        }
    })?;
}
}
 //--------------------fin ui--------------

 