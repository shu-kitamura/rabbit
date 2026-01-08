# rabbit

ターミナル上でアスキーアートのウサギを描画する、シンプルな Rust 製 CLI/TUI プロジェクトです。

## できること

- 一定間隔（デフォルト 30fps）で画面を更新して描画します
- `Actor` trait を通して「動くもの」を差し替えられる構成です（現状は `Rabbit` のみ）
- `rabbit.txt` のアスキーアートを、指定サイズに収まるように縮小して表示します
- ウサギは右方向にゆっくり移動します（現状: 折り返し/ループなし）

※ 現時点では CLI オプションはありません（設定は `AppConfig::default()` 固定です）。

## 動作環境

- Rust: `edition = "2024"`（`rustc 1.88` で動作確認）
- ANSI エスケープシーケンスが使える端末（macOS/Linux の一般的なターミナルを想定）

## 実行方法

リポジトリ直下で以下を実行します。

```bash
cargo run
```

インストールして `rabbit` コマンドとして使う場合:

```bash
cargo install --path .
rabbit
```

- 終了: `Ctrl+C`
- 画面表示が崩れた場合: `reset` もしくは `clear` を試してください

## 開発

```bash
cargo test
cargo fmt
cargo clippy
```

## 構成（ざっくり）

- `src/app.rs`: アプリ本体・メインループ
- `src/actor.rs`: 動物（Actor）の共通インターフェース
- `src/rabbit.rs`: ウサギの状態・描画（スプライト）
- `src/renderer.rs`: ターミナルへの描画
- `src/ascii.rs`: アスキーアートの読み込み/縮小
- `rabbit.txt`: 元となるアスキーアート
- `docs/design.md`, `docs/test-policy.md`: 設計とテスト方針

## ライセンス

MIT License（`LICENSE` を参照）
