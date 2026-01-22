# rust-book-manager

## 動作環境

Windowsで開発を行っています

- (必須) Docker, Docker Composeが利用可能であること
- (推奨) cargo makeが利用可能であること

## 使い方

- 初回インストール

 ```shell
 cargo make compose-up-db
 # Windowsの場合
 # cargo make migrate-with-ps
 # それ以外の場合
 # cargo make migrate-with-bash
 cargo make initial-setup
 cargo make compose-up-redis
 ```

- 環境起動
  - フロントエンド

    ```shell
    cargo make frontend-run-in-docker
    ```

    `http://localhost:3000` でアクセス可能なWebUIが起動します

  - バックエンド

    - Rust動作環境がローカルで構築されている場合
      - 主にこちらで動作を確認しています

    ```shell
    cargo make run
    ```

    - Rust動作環境がローカルにない場合
      - ローカルにRustをインストールせずに実行したい場合等に利用可能です。開発環境での検証が足りていないため、思わぬエラーが発生する可能性があります

    ```shell
    cargo make backend-run-in-docker
    ```

    `http://localhost:8080` でアクセス可能なAPIが起動します

## 出典

本リポジトリは、『Rust による Web アプリケーション開発』の作者提供の、以下リポジトリを元に構築しました

- [rust-web-app-book/rusty-book-manager-template](https://github.com/rust-web-app-book/rusty-book-manager-template)
