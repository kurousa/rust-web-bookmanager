use std::env;
use strum::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumString)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Environment {
    #[default]
    Development,
    Production,
}

impl Environment {
    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }
}

/// 環境判別を行い、Enum `Environment`を返す
/// - 環境変数`ENV`の設定値で分岐
///   - `production`と指定されていれば本番環境向け
///   - `development`と指定されていれば開発環境向け
///   - それ以外あるいは、未指定の場合は次の条件で判定
/// - アプリケーションのビルド状態を、`#[cfg(debug_assertions)` でチェックし、分岐
///   - デバッグビルドの場合は開発環境向け
///   - リリースビルドの場合は本番環境向け
///
pub fn which() -> Environment {
    // debug_assertionsがonの時は開発環境、offの時は本番環境とする
    #[cfg(debug_assertions)]
    let default_env = Environment::Development;
    #[cfg(not(debug_assertions))]
    let default_env = Environment::Production;

    match env::var("ENV") {
        Err(_) => default_env,
        Ok(v) => v.parse().unwrap_or(default_env),
    }
}

/// 本番環境かどうかを判定する
pub fn is_prod() -> bool {
    which().is_production()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    // 環境変数はプロセスグローバルなので、並列実行されると競合する
    // そのため、Mutexで保護して直列化する
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    fn lock_env() -> MutexGuard<'static, ()> {
        ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn test_which_development() {
        let _lock = lock_env();
        env::set_var("ENV", "development");
        assert_eq!(which(), Environment::Development);
        env::remove_var("ENV");
    }

    #[test]
    fn test_which_production() {
        let _lock = lock_env();
        env::set_var("ENV", "production");
        assert_eq!(which(), Environment::Production);
        env::remove_var("ENV");
    }

    #[test]
    fn test_which_invalid_env() {
        let _lock = lock_env();
        env::set_var("ENV", "invalid");
        // 無効な値の場合は、ビルドプロファイルに応じたデフォルト値が返る
        #[cfg(debug_assertions)]
        assert_eq!(which(), Environment::Development);
        #[cfg(not(debug_assertions))]
        assert_eq!(which(), Environment::Production);
        env::remove_var("ENV");
    }

    #[test]
    fn test_which_no_env() {
        let _lock = lock_env();
        env::remove_var("ENV");
        // ENV未指定の場合は、ビルドプロファイルに応じたデフォルト値が返る
        #[cfg(debug_assertions)]
        assert_eq!(which(), Environment::Development);
        #[cfg(not(debug_assertions))]
        assert_eq!(which(), Environment::Production);
    }

    #[test]
    fn test_environment_from_str() {
        use std::str::FromStr;
        assert_eq!(
            Environment::from_str("development"),
            Ok(Environment::Development)
        );
        assert_eq!(
            Environment::from_str("production"),
            Ok(Environment::Production)
        );
        assert_eq!(
            Environment::from_str("DEVELOPMENT"),
            Ok(Environment::Development)
        );
        assert_eq!(
            Environment::from_str("PRODUCTION"),
            Ok(Environment::Production)
        );
        assert!(Environment::from_str("invalid").is_err());
    }

    #[test]
    fn test_is_prod() {
        let _lock = lock_env();
        env::set_var("ENV", "production");
        assert!(is_prod());
        env::set_var("ENV", "development");
        assert!(!is_prod());
        env::remove_var("ENV");
    }
}
