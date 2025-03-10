# adapter

## 役割

データベースなどの永続化層へのアクセスを担う

## コンポーネント

- リポジトリ(repository)
  - `kernel`レイヤーで定義したインターフェイスの具象実装
- ミドルウェアへのアクセス(database, redis)
  - PostgresSQL,Redisへのアクセスに関する処理
