sixtyfps::include_modules!();

fn main() {
    let ui = AppWindow::new();

    ui.on_input_added(move |text| {
        println!("Input : {}",text.to_string());
    });

    ui.on_output_added(move |text| {
        println!("Output : {}",text.to_string());
    });

    ui.run();
}
