use libsql::{Builder, Connection};

use crate::config::app_config::AppConfig;

pub async fn new(conf: &AppConfig) -> Connection {
    let addr = format!("http://{}:{}", conf.db.host, conf.db.port);
    let db = Builder::new_remote(addr, "".to_string())
        .build()
        .await
        .unwrap();

    db.connect().unwrap()
}
