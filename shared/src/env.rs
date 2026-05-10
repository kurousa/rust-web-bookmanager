use std::env;
use strum::EnumString;

#[derive(Default, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    #[default]
    Development,
    Production,
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
