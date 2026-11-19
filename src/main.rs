use macroquad::prelude::*;

#[macroquad::main("reallycooldemo")]
async fn main(){
    let fontdata = include_bytes!("../DancingScriptRegular.ttf");
    let font = load_ttf_font_from_bytes(fontdata).unwrap();
     set_default_font(font);
    loop{
        clear_background(WHITE);
        draw_text("hi :o", 20.0, 40.0, 30.0, BLACK);
        next_frame().await;
    }
}