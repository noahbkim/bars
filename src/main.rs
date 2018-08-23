extern crate piston_window;

use piston_window::*;


fn draw(context: Context, graphics: &mut G2d) {
    clear([1.0; 4], graphics);
    rectangle([1.0, 0.0, 0.0, 1.0],
              [10.0, 10.0, 110.0, 110.0],
              context.transform,
              graphics);
}


fn main() {
    let settings: WindowSettings = WindowSettings::new("Bars", [640, 480])
        .exit_on_esc(true);
    let mut window: PistonWindow = settings.build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, &draw);
    }
}
