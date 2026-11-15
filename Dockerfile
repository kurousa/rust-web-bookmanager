FROM rust:slim-trixie AS builder

WORKDIR /app

# 静的リンク(musl)ビルドに必要なツールをインストール
RUN apt-get update && \
    apt-get install -y --no-install-recommends musl-tools && \
    rm -rf /var/lib/apt/lists/*

# 依存関係のコピー (ルートと各クレート)
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY adapter/Cargo.toml ./adapter/
COPY api/Cargo.toml ./api/
COPY kernel/Cargo.toml ./kernel/
COPY registry/Cargo.toml ./registry/
COPY shared/Cargo.toml ./shared/

# muslターゲットの追加と依存関係のダウンロード
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo fetch --target x86_64-unknown-linux-musl

# ソースコードのコピー
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

# 最終ステージにはDistroless staticイメージを使用
FROM gcr.io/distroless/static-debian13:nonroot
WORKDIR /app

# ビルダー段階からビルドされたアプリケーションをコピー
ARG APP_NAME=app
COPY --from=builder --chown=nonroot:nonroot /app/target/x86_64-unknown-linux-musl/release/${APP_NAME} /app

USER nonroot:nonroot

# 環境変数を設定
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}
ENV PORT=8080
EXPOSE ${PORT}

# ENTRYPOINTをexec形式で使用
ENTRYPOINT ["/app"]
