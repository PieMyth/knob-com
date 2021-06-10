extern crate find_folder;

#[cfg(target_os = "windows")]
fn main() -> Result<(), systray::Error> {    
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    // Load everything within the "assets" folder
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let icon = assets.join("knob.ico");
    app.set_icon_from_file(icon.to_str().unwrap())?;

    app.add_menu_item("Print a thing", |_| {
        println!("Printing a thing!");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Add Menu Item", |window| {
        window.add_menu_item("Interior item", |_| {
            println!("what");
            Ok::<_, systray::Error>(())
        })?;
        window.add_menu_separator()?;
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}