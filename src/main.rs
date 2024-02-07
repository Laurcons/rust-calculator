use controller::Controller;
use view::View;
pub mod view;
pub mod controller;

fn main() {
    let mut ctrl = Controller::new();
    let mut view = View::new(&mut ctrl);
    view.keypress_loop();
}
