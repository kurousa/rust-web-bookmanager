# --- ステージ1: 依存関係のインストール ---
FROM node:20-alpine AS deps
# Alpineでネイティブ依存関係が必要な場合（libc6-compat）の追加
# ビルドエラーの原因等によくあがるため、alpineをベースにする場合は導入が推奨される
RUN apk add --no-cache libc6-compat
WORKDIR /app

COPY package.json package-lock.json ./
# 開発用を含めた全ての依存関係をインストール（ビルドに必要）
RUN npm ci

# --- ステージ2: ビルド ---
FROM node:20-alpine AS builder
WORKDIR /app

# depsステージからnode_modulesをコピー
COPY --from=deps /app/node_modules ./node_modules
COPY . .

# Next.jsのビルド実行 (standaloneモードが有効であることを前提)
RUN npm run build

# --- ステージ3: 実行用（Runner） ---
FROM node:20-alpine AS runner
WORKDIR /app

# 環境変数の設定
ENV NODE_ENV production
ARG BACKEND_HOST
ENV BACKEND_HOST=${BACKEND_HOST}
ARG BACKEND_PORT
ENV BACKEND_PORT=${BACKEND_PORT}
ENV FRONTEND_PORT 3000

# セキュリティ: デフォルトのnodeユーザ（uid:1000）を使用するための設定
# WORKDIR内の所有権をnodeユーザに変更
RUN chown node:node /app

# ビルド成果物のうち、実行に必要な最小限のファイルのみをコピー
# standaloneモードでは .next/standalone に必要なnode_modulesも含まれます
COPY --from=builder --chown=node:node /app/public ./public
COPY --from=builder --chown=node:node /app/.next/standalone ./
COPY --from=builder --chown=node:node /app/.next/static ./.next/static

# 実行ユーザを node に切り替え
USER node

# ポートの公開
EXPOSE $FRONTEND_PORT
ENV PORT $FRONTEND_PORT

# npm startではなく、nodeで直接起動することで軽量・高速化
CMD ["node", "server.js"]
