enum DbType {
    Mongo,
    Postgre,
}

pub struct Endpoint<T> {
    port: i32,
    host: String,
    body_schema: T,
}
pub struct Service {
    Db: Option<DbType>,
}

enum OauthProviders {
    GitHub,
    Google,
    Facebook,
    Twitter,
}
pub struct AuthConfig {
    jwt: boolean,
    oath: Option<Vec<OathProviders>>,
}
pub struct Config {
    endpoints: Vec<Endpoint>,
    cache: boolean,
    auth: Option<AuthConfig>,
}
