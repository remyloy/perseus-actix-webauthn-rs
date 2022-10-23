pub struct Config<'a> {
    pub identity_url: &'a str,
    pub logout_url: &'a str,
    pub register_start: &'a str,
    pub register_finish: &'a str,
    pub login_start: &'a str,
    pub login_finish: &'a str,
}

pub const CONFIG: Config<'static> = Config {
    identity_url: "https://localhost/identity",
    logout_url: "https://localhost/logout",
    register_start: "https://localhost/register_start",
    register_finish: "https://localhost/register_finish",
    login_start: "https://localhost/login_start",
    login_finish: "https://localhost/login_finish",
};
