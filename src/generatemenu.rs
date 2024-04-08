use crate::{main, menu::MenuItem}; 
 
pub fn generateMenu() -> slint::VecModel<MenuItem>{
let subsubmenu_item = MenuItem::new(
    "Importa PDF".to_string(),
    Some("Ctrl+D".to_string()),
    Some(Box::new(|| println!("Deep Submenu Item action executed"))),
);

let submenu_item = MenuItem::with_submenu(
    "Importa".to_string(),
    Some("Ctrl+S".to_string()),
    vec![subsubmenu_item],
);

let main_menu = MenuItem::with_submenu(
    "File".to_string(),
    None,
    vec![submenu_item],
);
return slint::VecModel::from(vec![
main_menu
    ]);
//return main_menu;
}