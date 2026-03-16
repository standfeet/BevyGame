# Bevy App

Rust + Bevy で構築する Web アプリケーション。WASM にコンパイルし、ブラウザ上で動作します。

## 必要なツール

| ツール | インストール |
|---|---|
| Rust (stable) | https://rustup.rs/ |
| WASM ターゲット | `rustup target install wasm32-unknown-unknown` |
| Trunk | `cargo install trunk` |
| wrangler (デプロイ用) | `npm install -g wrangler` |

## セットアップ

```bash
git clone <repository-url>
cd bevy-app
```

依存クレートの初回ダウンロード・コンパイルには数分かかります。

## 開発

```bash
trunk serve
```

http://127.0.0.1:8080 で開発サーバーが起動します。`src/`、`assets/`、`index.html` の変更を検知して自動リビルド（ホットリロード）されます。

ポートを変更する場合:

```bash
trunk serve --port 3000
```

## ビルド

```bash
trunk build --release
```

`dist/` ディレクトリに成果物が出力されます。リリースビルドでは WASM サイズの最小化（`opt-level = "z"` + LTO）が適用されます。

## デプロイ (Cloudflare Pages)

### CLI からデプロイ

```bash
trunk build --release
npx wrangler pages deploy dist
```

### GitHub 連携による自動デプロイ

Cloudflare Pages ダッシュボードでリポジトリを接続し、以下を設定します:

| 項目 | 値 |
|---|---|
| ビルドコマンド | `trunk build --release` |
| 出力ディレクトリ | `dist` |

> Cloudflare のビルド環境に Rust と Trunk が必要です。ビルドイメージに含まれていない場合はインストールスクリプトを用意してください。

## 日本語テキストの表示

Bevy のデフォルトフォントは日本語グリフを含まないため、日本語対応フォントを別途用意する必要があります。

### 1. フォントを配置

`assets/fonts/` に日本語対応の TTF / OTF フォントを配置します。本プロジェクトでは [Noto Sans JP](https://github.com/notofonts/noto-cjk)（SIL OFL ライセンス）を使用しています。

### 2. AssetMetaCheck を無効化

WASM 環境では `.meta` ファイルの取得が 404 になりアセット読み込みが失敗します。`AssetPlugin` で `meta_check: AssetMetaCheck::Never` を設定してください。

```rust
use bevy::asset::AssetMetaCheck;

App::new()
    .add_plugins(
        DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }),
    )
```

### 3. フォントを読み込んでテキストに適用

```rust
let font = asset_server.load("fonts/NotoSansJP-Regular.otf");

commands.spawn((
    Text::new("こんにちは、Bevy！"),
    TextFont {
        font,
        font_size: 40.0,
        ..default()
    },
    TextColor(Color::WHITE),
));
```

### 4. Trunk にアセットをコピーさせる

`index.html` に以下を追加して、ビルド時に `assets/` が `dist/` にコピーされるようにします。

```html
<link data-trunk rel="copy-dir" href="assets" />
```

## プロジェクト構成

```
bevy-app/
├── .cargo/config.toml   # WASM をデフォルトターゲットに設定
├── Cargo.toml            # Bevy 0.18 (WebGL2)
├── Trunk.toml            # Trunk 開発サーバー設定
├── index.html            # Trunk エントリーポイント
├── wrangler.toml         # Cloudflare Pages 設定
├── assets/               # 画像・フォント等のアセット
└── src/
    └── main.rs           # アプリケーションのエントリーポイント
```

## 技術スタック

- **Bevy 0.18** - ECS ベースのゲームエンジン / アプリフレームワーク
- **WebGL2** - ブラウザ上での GPU レンダリング（iOS 含む幅広いブラウザに対応）
- **Trunk** - WASM ビルド & 開発サーバー
- **Cloudflare Pages** - 静的サイトホスティング
