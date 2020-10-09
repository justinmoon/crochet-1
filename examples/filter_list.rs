//! A simple list view example.

use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

use crochet::{AppHolder, Button, Column, Cx, DruidAppData, Label, Row};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder);
    let data = Default::default();
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}

#[derive(Default)]
struct MyAppLogic {
    numbers: Vec<u32>,
    exclude_primes: bool,
    exclude_evens: bool,
    exclude_squares: bool,
}

fn filter_label(exclude: bool, label: &str) -> String {
    if exclude {
        format!("Exclude {}", label)
    } else {
        format!("Include {}", label)
    }
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    true
}

fn is_even(n: u32) -> bool {
    n % 2 == 0
}

fn is_square(n: u32) -> bool {
    let sqrt = (n as f64).sqrt();
    sqrt.floor() == sqrt
}

impl MyAppLogic {
    fn run(&mut self, cx: &mut Cx) {
        Row::new().build(cx, |cx| {
            if Button::new(filter_label(self.exclude_primes, "Primes")).build(cx) {
                self.exclude_primes = !self.exclude_primes;
            }
            if Button::new(filter_label(self.exclude_evens, "Evens")).build(cx) {
                self.exclude_evens = !self.exclude_evens;
            }
            if Button::new(filter_label(self.exclude_squares, "Squares")).build(cx) {
                self.exclude_squares = !self.exclude_squares;
            }
        });
        Column::new().build(cx, |cx| {
            let numbers = self.numbers.iter().filter(|n| {
                if self.exclude_primes && is_prime(**n) {
                    false
                } else if self.exclude_evens && is_even(**n) {
                    false
                } else if self.exclude_squares && is_square(**n) {
                    false
                } else {
                    true
                }
            });
            for n in numbers {
                Label::new(n.to_string()).build(cx);
            }
        });
    }
}

fn ui_builder() -> impl Widget<DruidAppData> {
    let mut app_logic = MyAppLogic {
        numbers: (2..20).collect(),
        exclude_primes: false,
        exclude_evens: false,
        exclude_squares: false,
    };

    AppHolder::new(move |cx| app_logic.run(cx))
}
