//#[derive(Default, Clone)]
pub struct MenuItem {
    name: String,
    shortcut: Option<String>,
    callback: Option<Box<dyn Fn() + 'static>>,
    submenu: Option<Vec<MenuItem>>,
}

//#[derive(Default, Clone, Debug, slint::Model)]
struct UiMenuItem {
    name: String,
    shortcut: Option<String>,
}

impl MenuItem {
        // Function to convert a MenuItem (and its submenu, recursively) to UiMenuItem
        fn to_ui_menu_item(&self) -> UiMenuItem {
            UiMenuItem {
                name: self.name.clone(),
                shortcut: self.shortcut.clone(),
            }
        }
    
     
        pub fn new(name: String, shortcut: Option<String>, callback: Option<Box<dyn Fn() + 'static>>) -> Self {
        MenuItem {
            name,
            shortcut,
            callback,
            submenu: None,
        }
    }

    pub    fn with_submenu(name: String, shortcut: Option<String>, submenu: Vec<MenuItem>) -> Self {
        MenuItem {
            name,
            shortcut,
            callback: None,
            submenu: Some(submenu),
        }
    }

    fn execute(&self) {
        if let Some(ref callback) = self.callback {
            callback();
        } else if let Some(ref submenu) = self.submenu {
            println!("{} opens a submenu.", self.name);
            // Optionally list submenu items or handle further interaction
            for item in submenu {
                println!("- {}", item.name);
            }
        }
    }
}