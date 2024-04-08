use generatemenu::generateMenu;

slint::include_modules!();
mod menu; // This tells Rust to load the code from `menu.rs` as a module.
use menu::MenuItem; 

pub mod generatemenu;

fn main() -> Result<(), slint::PlatformError> {
    let menu =generateMenu();
    let ui = AppWindow::new()?;
//    ui.set_menu_items(menu);
//ui.set_menu_items(slint::ModelRc::new(menu));

//let ui_menu_items: Vec<UiMenuItem> = menu.iter().map(|item| item.to_ui_menu_item()).collect();
 
//let model = slint::VecModel::from(ui_menu_items);

//ui.set_menu_items(slint::ModelRc::new(model.clone()));
ui.run()
}
