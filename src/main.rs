use macroquad::prelude::*;

#[macroquad::main("reallycooldemo")]
async fn main(){
    let fontdata = include_bytes!("../DancingScriptRegular.ttf");
    let font = load_ttf_font_from_bytes(fontdata).unwrap();
     set_default_font(font);
    loop{
        draw_text("hi mr.davis :o", 0.0, 0.0, 10.0, WHITE);
    }
}