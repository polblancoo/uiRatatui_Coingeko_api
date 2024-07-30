use tabled::{ Style, Table, Tabled};
use crate::api_call::Coin;
 
#[derive(Tabled)]
struct SelectedFields<'a> {
    id: &'a str,
    symbol: &'a str,
    name: &'a str,
    current_price: f64,
    market_cap: u64,
    market_cap_rank: u32,
}

pub fn print_coins_result01(result: Vec<Coin>) {
   // Crear un nuevo vector con los campos seleccionados
   let selected_fields: Vec<SelectedFields> = result
        .iter()
        .map(|coin| SelectedFields {
            id: &coin.id,
            symbol: &coin.symbol,
            name: &coin.name,
            current_price: coin.current_price,
            market_cap: coin.market_cap,
            market_cap_rank: coin.market_cap_rank,
        })
        .collect();
  print_coins(selected_fields) 
} 

fn print_coins(coins: Vec<SelectedFields>) {
    // Convierte Vec<Coin> a Table
    let table = Table::new(coins)
    .with(Style::PSEUDO);
  
   // Imprimir la tabla en la terminal
   println!("{}", table);
}
