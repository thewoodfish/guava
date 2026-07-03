use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub circuits_dir: String,
    pub jwt_secret: String,
    pub port: u16,
    pub soroban_contract_id: String,
    pub stellar_identity: String,
    pub stellar_network: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Arc<Self>> {
        load_dotenv();
        let circuits_dir =
            std::env::var("CIRCUITS_DIR").unwrap_or_else(|_| "circuits/lending".to_string());

        Ok(Arc::new(Self {
            database_url: required("DATABASE_URL")?,
            circuits_dir: resolve_path(&circuits_dir).to_string_lossy().into_owned(),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "ledgerproof-dev-secret-2026-change-in-prod".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()?,
            soroban_contract_id: std::env::var("SOROBAN_CONTRACT_ID").unwrap_or_default(),
            stellar_identity: std::env::var("STELLAR_IDENTITY")
                .unwrap_or_else(|_| "alice".to_string()),
            stellar_network: std::env::var("STELLAR_NETWORK")
                .unwrap_or_else(|_| "testnet".to_string()),
        }))
    }
}

fn load_dotenv() {
    if dotenvy::dotenv().is_ok() {
        return;
    }

    let repo_env = Path::new(env!("CARGO_MANIFEST_DIR")).join("../.env");
    let _ = dotenvy::from_path(repo_env);
}

fn resolve_path(raw: &str) -> PathBuf {
    let path = Path::new(raw);
    if path.is_absolute() {
        return path.to_path_buf();
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let candidates = [
        std::env::current_dir().ok().map(|cwd| cwd.join(path)),
        Some(manifest_dir.join(path)),
        manifest_dir.parent().map(|repo_root| repo_root.join(path)),
    ];

    candidates
        .into_iter()
        .flatten()
        .find(|candidate| candidate.exists())
        .unwrap_or_else(|| path.to_path_buf())
}

fn required(key: &str) -> anyhow::Result<String> {
    std::env::var(key).map_err(|_| anyhow::anyhow!("missing env var: {key}"))
}
