use anyhow::Result;

pub struct AppConfig {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub auth: AuthConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let database = DatabaseConfig {
            host: std::env::var("DATABASE_HOST")?,
            port: std::env::var("DATABASE_PORT")?.parse()?,
            username: std::env::var("DATABASE_USERNAME")?,
            password: std::env::var("DATABASE_PASSWORD")?,
            database: std::env::var("DATABASE_NAME")?,
        };
        let redis = RedisConfig {
            host: std::env::var("REDIS_HOST")?,
            port: std::env::var("REDIS_PORT")?.parse::<u16>()?,
        };
        let auth = AuthConfig {
            ttl: std::env::var("AUTH_TOKEN_TTL")?.parse::<u64>()?,
        };
        Ok(Self {
            database,
            redis,
            auth,
        })
    }
}

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

pub struct RedisConfig {
    pub host: String,
    pub port: u16,
}

pub struct AuthConfig {
    pub ttl: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    // Environment variables are global state, so we use a Mutex to prevent
    // tests from interfering with each other when running in parallel.
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    fn lock_env() -> MutexGuard<'static, ()> {
        ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn test_app_config_new() {
        let _lock = lock_env();

        // Set up the environment variables
        std::env::set_var("DATABASE_HOST", "localhost");
        std::env::set_var("DATABASE_PORT", "5432");
        std::env::set_var("DATABASE_USERNAME", "test_user");
        std::env::set_var("DATABASE_PASSWORD", "test_pass");
        std::env::set_var("DATABASE_NAME", "test_db");
        std::env::set_var("REDIS_HOST", "127.0.0.1");
        std::env::set_var("REDIS_PORT", "6379");
        std::env::set_var("AUTH_TOKEN_TTL", "3600");

        // Parse the configuration
        let config = AppConfig::new().expect("Failed to create AppConfig");

        // Assert database config
        assert_eq!(config.database.host, "localhost");
        assert_eq!(config.database.port, 5432);
        assert_eq!(config.database.username, "test_user");
        assert_eq!(config.database.password, "test_pass");
        assert_eq!(config.database.database, "test_db");

        // Assert redis config
        assert_eq!(config.redis.host, "127.0.0.1");
        assert_eq!(config.redis.port, 6379);

        // Assert auth config
        assert_eq!(config.auth.ttl, 3600);

        // Clean up the environment variables
        std::env::remove_var("DATABASE_HOST");
        std::env::remove_var("DATABASE_PORT");
        std::env::remove_var("DATABASE_USERNAME");
        std::env::remove_var("DATABASE_PASSWORD");
        std::env::remove_var("DATABASE_NAME");
        std::env::remove_var("REDIS_HOST");
        std::env::remove_var("REDIS_PORT");
        std::env::remove_var("AUTH_TOKEN_TTL");
    }

    #[test]
    fn test_app_config_new_missing_env() {
        let _lock = lock_env();

        // Ensure at least one variable is missing
        std::env::remove_var("DATABASE_HOST");

        // AppConfig::new() should fail
        let config = AppConfig::new();
        assert!(config.is_err());
    }
}
