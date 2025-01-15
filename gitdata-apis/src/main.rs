use actix_settings::ApplySettings;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;
use gitdata_apis::service::AppState;
use tracing::info;

const CONFIG_FILE : &str = "./config/api.toml";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    info!("Starting server");
    info!("Loading Api configuration");
    let api_config = if let Ok(api_config) = actix_settings::Settings::parse_toml(CONFIG_FILE) {
        api_config
    } else {
        actix_settings::Settings::write_toml_file(CONFIG_FILE)?;
        return Err(anyhow::anyhow!("Please configure the configuration file"));
    };
    let state = AppState::init().await?;
    HttpServer::new(move || App::new().app_data(web::Data::new(state.clone())))
        .try_apply_settings(&api_config)
        .expect("Failed to apply settings")
        .run()
        .await
        .expect("Failed to start server");
    Ok(())
}
