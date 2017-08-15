extern crate cursive;
extern crate clap;
extern crate error_chain;

use cursive::Cursive;
use cursive::views::{Dialog, TextView};

fn main() {
    let mut ui = Cursive::new();

    ui.add_layer(Dialog::around(TextView::new("Hello from Manifestor!"))
                             .title("Manifestor")
                             .button("Quit", |s| s.quit()));

    ui.run();
}
