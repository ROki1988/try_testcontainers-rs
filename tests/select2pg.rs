use testcontainers::*;
use testcontainers::images::postgres::Postgres;
use diesel::{PgConnection, Connection};

#[test]
fn connect2pg() {
    let _ = pretty_env_logger::try_init();
    let docker = clients::Cli::default();
    let pg = docker.run(Postgres::default());
    let host_port = pg.get_host_port(5432).unwrap();
    let url = format!("postgres://localhost:{}", host_port);
    let client = PgConnection::establish(url.as_str());
    assert!(client.is_ok());
}