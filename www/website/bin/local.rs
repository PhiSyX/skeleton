use app::App;

#[project::setup(
    config = App::APP_NAME,
    logger = "stdout",
    database = "sqlite"
)]
async fn main<AsyncRuntime>(
    args: app::cli_app,
    env: app::env_app,
) -> app::Result<()>
where
    AsyncRuntime: tokio,

    [config]: config::www::Website,
    [database]: database::services::SqliteService,
{
    let config_server = config.server;
	let site = App::new(cli_args, env_args, config_server, database);
	site.launch().await
}
