use raylib::prelude::*;
use crate::interpreter::Interpreter;

pub fn run(_machine: &Interpreter) {    
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("VM1")
        .build();

    // let font = rl.load_font_ex(&thread, "res/fonts/dogica.ttf", 22, FontLoadEx::Default(1000)).expect("couldn't load font");
    let font = rl.load_font_ex(&thread, "res/fonts/dogica.ttf", 16, FontLoadEx::Default(2000)).expect("couldn't get font");
    
    rl.set_target_fps(60);

    let mut text = String::new();

    // for i in 0..80 {
    //     text.push_str(&format!("{}", i % 10)[..]);
    // }
    for j in 0..40 {
        // text.push_str(&format!("\n{}", j)[..]);
        for i in j..40 {
            text.push_str(&format!("{}", i % 10)[..]);
        }
        text.push_str("\n");
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        // d.draw_text_ex(&font, &text[..], Vector2::zero(), 14.0, 0.0, Color::WHITE);
        for i in 0..40 {
            for j in 0..40 {
                let text = &format!("{}", j % 10)[..];
                d.draw_text_ex(&font, text, Vector2::new(j as f32* 20.0, i as f32 * 20.0), font.base_size() as f32, 0.0, Color::WHITE);
            }
        }
    }
}