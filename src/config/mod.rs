use redis::{Client, RedisResult, aio::ConnectionManager};

#[derive(Debug, Clone)]
pub struct RedisConfig {
    host: String,
    port: u16,
    use_tls: bool,
    insecure: bool,
    password: Option<String>,
    db: Option<u8>,
}

impl RedisConfig {
    pub fn builder() -> RedisConfigBuilder {
        RedisConfigBuilder::default()
    }

    fn build_url(&self) -> String {
        let scheme = if self.use_tls { "rediss" } else { "redis" };
        let auth = self
            .password
            .as_ref()
            .map(|p| format!(":{}@", p))
            .unwrap_or_default();
        let db = self.db.map(|d| format!("/{}", d)).unwrap_or_default();
        let insecure = if self.use_tls && self.insecure {
            "#insecure"
        } else {
            ""
        };

        format!(
            "{}://{}{}:{}{}{}",
            scheme, auth, self.host, self.port, db, insecure
        )
    }

    pub async fn connect(&self) -> RedisResult<ConnectionManager> {
        let url = self.build_url();
        let client = Client::open(url)?;
        client.get_connection_manager().await
    }
}

#[derive(Debug, Clone)]
pub struct RedisConfigBuilder {
    host: String,
    port: u16,
    use_tls: bool,
    insecure: bool,
    password: Option<String>,
    db: Option<u8>,
}

impl Default for RedisConfigBuilder {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 6380,
            use_tls: true,
            insecure: true,
            password: None,
            db: Some(0),
        }
    }
}

impl RedisConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn tls(mut self, insecure: bool) -> Self {
        self.use_tls = true;
        self.insecure = insecure;
        if self.port == 6379 {
            self.port = 6380;
        }
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn db(mut self, db: u8) -> Self {
        self.db = Some(db);
        self
    }

    pub fn build(self) -> RedisConfig {
        RedisConfig {
            host: self.host,
            port: self.port,
            use_tls: self.use_tls,
            insecure: self.insecure,
            password: self.password,
            db: self.db,
        }
    }

    pub async fn connect(self) -> RedisResult<ConnectionManager> {
        self.build().connect().await
    }
}
