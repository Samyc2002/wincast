pub enum CurrentScreen {
    Home,
    Search,
}

pub struct App {
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Home,
        }
    }

    pub fn toggle_screen(&mut self) {
        match self.current_screen {
            CurrentScreen::Home => self.current_screen = CurrentScreen::Search,
            CurrentScreen::Search => self.current_screen = CurrentScreen::Home,
        }
    }
}
