use clap::Args;
use color_eyre::{eyre::eyre, Result};
use tokio::net::TcpListener;

use crate::router;

#[derive(Debug, Args)]
pub struct RunServerArgs {
    #[arg(long, env = "MONOCLE_HOST")]
    host: String,
    #[arg(long, env = "MONOCLE_PORT")]
    port: u16,
}

impl RunServerArgs {
    pub async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind((self.host.clone(), self.port)).await?;
        axum::serve(listener, router::routes())
            .await
            .map_err(|e| eyre!("error launching server {}", e))
    }
}
