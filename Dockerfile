FROM rust:slim-trixie AS builder

WORKDIR /app

# 依存関係のコピー (ルートと各クレート)
COPY Cargo.toml Cargo.lock ./
COPY adapter/Cargo.toml ./adapter/
COPY api/Cargo.toml ./api/
COPY kernel/Cargo.toml ./kernel/
COPY registry/Cargo.toml ./registry/
COPY shared/Cargo.toml ./shared/

# 依存関係のダウンロード
RUN cargo fetch

# ソースコードのコピー
COPY . .

RUN cargo build --release

# 最終ステージには小さなベースイメージを使用
FROM debian:trixie-slim
WORKDIR /app

RUN adduser book && chown -R book /app
USER book

# ビルダー段階からビルドされたアプリケーションをコピー
ARG APP_NAME=app
COPY --from=builder /app/target/release/${APP_NAME} /app/${APP_NAME}

# 環境変数を設定
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}
ENV PORT=8080
EXPOSE ${PORT}

# ENTRYPOINTをexec形式で使用
ENTRYPOINT ["/app/${APP_NAME}"]
