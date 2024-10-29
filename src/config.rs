#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum CargoEnv {
    Development,
    Testing,
    Production,
}

#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env, value_enum, default_value_t=CargoEnv::Development)]
    pub cargo_env: CargoEnv,

    #[clap(long, env, default_value = "9001")]
    pub port: u16,

    #[clap(long, env, default_value = "0.0.0.0")]
    pub host: String,

    #[clap(long, env, default_value = "app.db")]
    pub database_url: String,

    #[clap(long, env, default_value = "notagoodsecret")]
    pub shared_secret: String,
}
