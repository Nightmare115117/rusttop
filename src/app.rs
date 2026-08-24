pub struct App {
    app_status:String,
    running: bool,
}

impl App {
    pub fn new(app_status: String, running: bool ) -> App {
        App {
            app_status: app_status,
            running: running
        }
    }

    #[allow(dead_code)]
    pub fn get_app_status(&self) -> &str {
        return &self.app_status;
    }

    #[allow(dead_code)]
    pub fn is_running(&self) -> bool {
        return self.running;
    }

    #[allow(dead_code)]
    pub fn set_app_status(&mut self, app_status: String) {
        self.app_status = app_status;
    } 

    #[allow(dead_code)]
    pub fn set_running(&mut self, running: bool) {
        self.running = running;
    }
}