mod app;
mod dir;
mod img;

use crate::app::App;

fn main() {
    if let Err(ref errs) = App::new().run() {
        let mut errs = errs.iter();
        match errs.next() {
            Some(err) => {
                eprintln!("{}", err);
                errs.for_each(|err| eprintln!(" ↳ {}", err));
            }
            None => (),
        }
    }
}
