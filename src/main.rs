#![feature(conservative_impl_trait)]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate regex;

mod render;
mod fillbar;
mod w;

use std::iter;
use render::Elem;
pub use fillbar::FillBar;

// error & result type
#[derive(Debug)]
pub enum Error { CmdFailed, Utf8Error, BadCmdFormat(&'static str) }
pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        use Error::*;

        match *self {
            CmdFailed => write!(f, "command failed to execute"),
            Utf8Error => write!(f, "encountered invalid utf8 string"),
            BadCmdFormat(s) => write!(f, "bad command output: {}", s),
        }
    }
}

// the status bar's state (cooldowns and widget display caches)
struct StatusBar {
    widgets: Vec<Box<w::Widget>>,
    timers: Vec<i32>,
    cache: Vec<Vec<Elem>>,
}

impl StatusBar {
    fn new() -> Self {
        let widgets = all_widgets();
        let timers = widgets.iter().map(|_| 0).collect();
        let cache = widgets.iter().map(|_| vec![]).collect();

        StatusBar { widgets, timers, cache }
    }

    // update timers and re-render
    fn step(&mut self) -> Result<()> {
        for i in 0..self.timers.len() {
            if self.timers[i] == 0 {
                self.cache[i] = self.widgets[i].blit()?;
                self.timers[i] = self.widgets[i].cooldown();
            }
            self.timers[i] -= 1;
        }
        Ok(())
    }

    // returns the list of elements to render
    fn blit(&self) -> Vec<Elem> {
        let spacer = Elem::plain("    ");

        self.cache
            .iter()
            .flat_map(|elems| {
                elems.iter()
                     .cloned()
                     .chain(iter::once(spacer.clone()))
            })
            .collect()
    }
}


fn all_widgets() -> Vec<Box<w::Widget>> {
    vec![
        Box::new(w::DateTimeW::new()),
        Box::new(w::BatteryW::new()),
        Box::new(w::MemoryW::new()),
    ]
}


fn main() {
    let step_and_blit = |sb: &mut StatusBar| {
        match sb.step() {
            Ok(()) => sb.blit(),

            Err(e) => vec![
                Elem::new(format!("error: {}   ", e), "#ff0000")
            ],
        }
    };

    let make_blit_stream = || {
        iter::repeat(())
            .scan(StatusBar::new(), |sb, _| Some(step_and_blit(sb)))
    };

    render::render_stream(2000, make_blit_stream()).unwrap();
}
