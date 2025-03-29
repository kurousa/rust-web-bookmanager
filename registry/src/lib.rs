use std::sync::Arc;

use adapter::{
    database::ConnectionPool,
    redis::RedisClient,
    repository::{
        auth::AuthRepositoryImpl, checkout::CheckoutRepositoryImpl,
        health::HealthCheckRepositoryImpl, user::UserRepositoryImpl,
    },
};

use adapter::repository::book::BookRepositoryImpl;
use kernel::repository::{
    auth::AuthRepository, book::BookRepository, checkout::CheckoutRepository,
    health::HealthCheckRepository, user::UserRepository,
};

use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<dyn HealthCheckRepository>,
    book_repository: Arc<dyn BookRepository>,
    auth_repository: Arc<dyn AuthRepository>,
    user_repository: Arc<dyn UserRepository>,
    check_out_repository: Arc<dyn CheckoutRepository>,
}

impl AppRegistry {
    pub fn new(
        pool: ConnectionPool,
        redis_client: Arc<RedisClient>,
        app_config: AppConfig,
    ) -> Self {
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let book_repository = Arc::new(BookRepositoryImpl::new(pool.clone()));
        let auth_repository = Arc::new(AuthRepositoryImpl::new(
            pool.clone(),
            redis_client.clone(),
            app_config.auth.ttl,
        ));
        let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone()));
        let check_out_repository = Arc::new(CheckoutRepositoryImpl::new(pool.clone()));
        Self {
            health_check_repository,
            book_repository,
            auth_repository,
            user_repository,
            check_out_repository,
        }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }

    pub fn book_repository(&self) -> Arc<dyn BookRepository> {
        self.book_repository.clone()
    }

    pub fn auth_repository(&self) -> Arc<dyn AuthRepository> {
        self.auth_repository.clone()
    }

    pub fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }

    pub fn check_out_repository(&self) -> Arc<dyn CheckoutRepository> {
        self.check_out_repository.clone()
    }
}
