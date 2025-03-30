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
pub struct AppRegistryImpl {
    health_check_repository: Arc<dyn HealthCheckRepository>,
    book_repository: Arc<dyn BookRepository>,
    auth_repository: Arc<dyn AuthRepository>,
    user_repository: Arc<dyn UserRepository>,
    check_out_repository: Arc<dyn CheckoutRepository>,
}

impl AppRegistryImpl {
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
}

use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait AppRegistryExt {
    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository>;
    fn book_repository(&self) -> Arc<dyn BookRepository>;
    fn auth_repository(&self) -> Arc<dyn AuthRepository>;
    fn user_repository(&self) -> Arc<dyn UserRepository>;
    fn check_out_repository(&self) -> Arc<dyn CheckoutRepository>;
}
impl AppRegistryExt for AppRegistryImpl {
    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }

    fn book_repository(&self) -> Arc<dyn BookRepository> {
        self.book_repository.clone()
    }

    fn auth_repository(&self) -> Arc<dyn AuthRepository> {
        self.auth_repository.clone()
    }

    fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }

    fn check_out_repository(&self) -> Arc<dyn CheckoutRepository> {
        self.check_out_repository.clone()
    }
}

//　従来AppRegistry型に依存していた処理に対し、
// トレイト実装された型として挿入するために型エイリアスで、AppRegistryを再定義する
pub type AppRegistry = Arc<dyn AppRegistryExt + Send + Sync + 'static>;
