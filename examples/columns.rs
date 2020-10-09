//! The classic counter example.

use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

use crochet::{AppHolder, Button, Cx, DruidAppData, Label, Row};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder);
    let data = Default::default();
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}

#[derive(Default)]
struct MyAppLogic {
    columns: Vec<String>,
}

impl MyAppLogic {
    fn run(&mut self, cx: &mut Cx) {
        if Button::new("+").build(cx) {
            println!("click: {:?}", self.columns);
            self.columns.push(format!("#{}", self.columns.len() + 1));
        }
        Row::new().build(cx, |cx| {
            for column in &self.columns {
                Label::new(column).build(cx);
            }
        });
    }
}

fn ui_builder() -> impl Widget<DruidAppData> {
    let mut app_logic = MyAppLogic::default();

    AppHolder::new(move |cx| app_logic.run(cx))
}
