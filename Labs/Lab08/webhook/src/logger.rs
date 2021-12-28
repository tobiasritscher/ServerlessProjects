use std::{fmt, io::Write, sync::atomic};

use env_logger::fmt::{Color, Style, StyledValue};
use log::Level;

pub fn setup(filter: &str) {
    env_logger::builder()
        .format(|buf, rec| {
            let module = rec.target();

            let module = Padded {
                value: module,
                width: max_target_width(module),
            };

            let time = buf.timestamp_millis();

            let style = &mut buf.style();
            let level = colored_level(style, rec.level());

            writeln!(buf, "[{} {} {}] {}", time, level, module, rec.args())
        })
        .parse_filters(filter)
        .init();
}

fn colored_level(style: &mut Style, level: Level) -> StyledValue<'_, &str> {
    let (sty, name) = match level {
        Level::Trace => (Color::Magenta, "TRACE"),
        Level::Debug => (Color::Blue, "DEBUG"),
        Level::Info => (Color::Green, "INFO "),
        Level::Warn => (Color::Yellow, "WARN "),
        Level::Error => (Color::Red, "ERROR"),
    };

    style.set_color(sty).value(name)
}

struct Padded<T> {
    value: T,
    width: usize,
}

impl<T: fmt::Display> fmt::Display for Padded<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <width$}", self.value, width = self.width)
    }
}

static MAX_MODULE_WIDTH: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

fn max_target_width(target: &str) -> usize {
    let max_width = MAX_MODULE_WIDTH.load(atomic::Ordering::Relaxed);
    if max_width < target.len() {
        MAX_MODULE_WIDTH.store(target.len(), atomic::Ordering::Relaxed);
        target.len()
    } else {
        max_width
    }
}
