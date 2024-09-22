pub struct Control {
    pub key: String,
    pub description: String,
}

pub fn get_controls() -> Vec<Control> {
    return vec![
        // Control {
        //     key: "▲ ▼".to_string(),
        //     description: "Scroll".to_string(),
        // },
        Control {
            key: "Up/Down".to_string(),
            description: "Scroll".to_string(),
        },
        Control {
            key: "<Ctrl>+s".to_string(),
            description: "Syncronize".to_string(),
        },
        // Control {
        //     key: "<Ctrl> 1".to_string(),
        //     description: "Switch to Apps".to_string(),
        // },
        // Control {
        //     key: "<Ctrl> 2".to_string(),
        //     description: "Switch to Messages".to_string(),
        // },
        Control {
            key: "<Ctrl>+g".to_string(),
            description: "Search Google".to_string(),
        },
        Control {
            key: "<Ctrl>+y".to_string(),
            description: "Search YouTube".to_string(),
        },
        Control {
            key: "<Ctrl>+m".to_string(),
            description: "Search YouTube Music".to_string(),
        },
        Control {
            key: "<Esc>".to_string(),
            description: "Exit".to_string(),
        },
    ];
}
