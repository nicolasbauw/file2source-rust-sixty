use std::process;
sixtyfps::include_modules!();

fn main() {
    let ui = AppWindow::new();

    ui.on_input_added(move |text| {
        println!("Input : {}",text.to_string());
    });

    ui.on_output_added(move |text| {
        println!("Output : {}",text.to_string());
    });

    ui.on_array_name(move |text| {
        println!("Array name : {}",text.to_string());
    });

    ui.on_amiga_changed(move |amiga| {
        println!("Amiga mode : {}",amiga);
    });

    ui.on_quit_clicked( || {
        process::exit(0);
    });

    ui.run();
}
