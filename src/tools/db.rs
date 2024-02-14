use libsql::{Connection, Database};

use crate::config::app_config::AppConfig;

pub fn new(conf: &AppConfig) -> Connection {
    let addr = format!("http://{}:{}", conf.db.host, conf.db.port);
    let db = Database::open_remote(addr, "").unwrap();

    db.connect().unwrap()
}
