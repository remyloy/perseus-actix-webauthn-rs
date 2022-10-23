#[derive(Clone)]
pub struct Config {
    pub endpoint: String,
    pub rp_id: String,
    pub rp_origin: String,
    pub redirect_logout: String,
}
