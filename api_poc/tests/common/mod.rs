use mongodb::Client;

use std::time::Duration;

use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};

pub const TEST_DB_NAME: &str = "test_db";
const TEST_USERNAME: &str = "root";
const TEST_PASSWORD: &str = "root";
const DEFAULT_MONGO_PORT: u16 = 27017;

pub async fn get_mongo_client_with_container() -> (ContainerAsync<GenericImage>, Client) {
    let container = GenericImage::new("mongo", "7.0")
        .with_wait_for(WaitFor::message_on_stdout("mongod startup complete"))
        .with_exposed_port(DEFAULT_MONGO_PORT.tcp())
        // configure mongo container
        .with_env_var("MONGO_INITDB_DATABASE", TEST_DB_NAME)
        .with_env_var("MONGO_INITDB_ROOT_USERNAME", TEST_USERNAME)
        .with_env_var("MONGO_INITDB_ROOT_PASSWORD", TEST_PASSWORD)
        // startup and network config
        .with_startup_timeout(Duration::from_secs(30))
        .start()
        .await
        .expect("Mongo container must be started");

    let dynamic_port = &container
        .ports()
        .await
        .expect("Mongo container must have ports")
        .map_to_host_port_ipv4(DEFAULT_MONGO_PORT.tcp())
        .expect("mongo container must have 27017 exposed");

    let conn_string = format!("mongodb://{TEST_USERNAME}:{TEST_PASSWORD}@localhost:{dynamic_port}/{TEST_DB_NAME}?directConnection=true&authSource=admin");

    let mongo_client = Client::with_uri_str(conn_string)
        .await
        .expect("Mongo client must be created");

    (container, mongo_client)
}
