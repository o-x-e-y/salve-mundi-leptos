use leptos::*;

#[derive(Debug, Clone, Copy)]
pub struct User {
    pub username: RwSignal<String>,
    // very mindful, very secure
    pub password: RwSignal<String>,
    logged_in: RwSignal<bool>
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        Self { username: create_rw_signal(username), password: create_rw_signal(password), logged_in: create_rw_signal(false) }
    }

    pub fn update(&self, username: String, password: String) {
        self.username.set(username);
        self.password.set(password);
    }

    pub fn logged_in(&self) -> bool {
        self.logged_in.get()
    }

    pub fn log_in(&self) {
        self.logged_in.set(true)
    }

    pub fn log_out(&self) {
        self.logged_in.set(false)
    }
}