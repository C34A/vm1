use raylib::prelude::*;
use crate::interpreter::Interpreter;
use crate::machine::val_to_char;

pub fn run(machine: &mut Interpreter) {    
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("VM1")
        .build();

    // let font = rl.load_font_ex(&thread, "res/fonts/dogica.ttf", 22, FontLoadEx::Default(1000)).expect("couldn't load font");
    let font = rl.load_font_ex(&thread, "res/fonts/dogica.ttf", 16, FontLoadEx::Default(2000)).expect("couldn't get font");
    
    rl.set_target_fps(144);

    while !rl.window_should_close() {

        machine.interpret_one();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        // d.draw_text_ex(&font, &text[..], Vector2::zero(), 14.0, 0.0, Color::WHITE);
        for (i, x) in machine.get_framebuffer().iter().enumerate() {
            d.draw_text_ex(&font, &val_to_char(*x).to_string()[..], Vector2::new((i % 40) as f32 * 20.0, (i / 40) as f32 * 20.0), 14.0, 0.0, Color::WHITE);
        }
    }
}