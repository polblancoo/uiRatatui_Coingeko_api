use ratatui::{
    text::Span,
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    layout::Rect,
    widgets::{Block, Borders, List, ListItem,Paragraph, Wrap},
    Frame,
};

use ratatui::text::Text;
use crate::api_call::Coin;

pub fn draw<B: Backend>(f: &mut Frame, coins: &[Coin], selected_index: usize ) {
    // Crear el layout principal con dos columnas
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(f.size());

    // Subdividir la columna derecha en dos filas
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(main_chunks[1]);

    // Frame 1: Lista de tokens (ocupa toda la altura a la izquierda)
    let tokens_block = Block::default()
        .title("Tokens")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    f.render_widget(tokens_block, main_chunks[0]);
   
    // Renderizar la lista de tokens
    let tokens_list = render_tokens_list(coins, selected_index);
    f.render_widget(tokens_list, main_chunks[0]);



    // Frame 2: Detalles del token seleccionado (arriba a la derecha)
    let details_block = Block::default()
        .title("Token Details")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));
    f.render_widget(details_block, right_chunks[0]);
        // Renderizar los detalles del token seleccionado
        if let Some(selected_coin) = coins.get(selected_index) {
            render_token_details::<B>(f, selected_coin, right_chunks[0]);
        }
    // Frame 3: Gráfico (abajo a la derecha)
    let graph_block = Block::default()
        .title("Price Graph")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));
    f.render_widget(graph_block, right_chunks[1]);
}

 fn render_tokens_list(coins: &[Coin], selected_index: usize) -> List<'static> {
    let tokens_list: Vec<ListItem> = coins
        .iter()
        .enumerate()
        .map(|(index, coin)| {
            let content = format!("{}: ${:.2}", coin.name, coin.current_price);
            let style = if index == selected_index {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(content, style))
        })
        .collect();

    List::new(tokens_list)
        .block(Block::default().title("Tokens").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
}

pub fn render_token_details<B: Backend>(f: &mut Frame, coin: &Coin, area: Rect) {
    let details = vec![
        format!("Name: {}", coin.name),
        format!("Symbol: {}", coin.symbol),
        format!("Price: ${:.2}", coin.current_price),
        format!("Market Cap: ${}", coin.market_cap),
        format!("24h Change: {:.2}%", coin.price_change_percentage_24h),
        format!("Volume: ${}", coin.total_volume),
        format!("Circulating Supply: {}", coin.circulating_supply),
        format!("Total Supply: {}", coin.total_supply),
        format!("Max Supply: {}", coin.max_supply.map_or("N/A".to_string(), |v| v.to_string())),
        format!("ATH: ${:.2}", coin.ath),
        format!("ATH Change %: {:.2}%", coin.ath_change_percentage),
        format!("ATH Date: {}", coin.ath_date),
    ];

    let details_text = Text::from(details.join("\n"));
    let details_paragraph = Paragraph::new(details_text)
        .block(Block::default().title("Token Details").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    f.render_widget(details_paragraph, area);
}