# 設計: CLI Rabbit

## 1. 全体アーキテクチャ
- CLI 入口で引数を解釈し、アプリ設定を生成する
- アニメーションループが「状態更新」と「描画」を一定間隔で実行する
- 描画はターミナルへの出力に限定し、アニメーションはモデル層で管理する
- 将来の動物追加に備えて、動物の振る舞いはトレイトで抽象化する
- 例外や終了処理はアプリ全体で一元管理する

## 2. モジュール構成案
- `src/main.rs`
  - エントリポイント、引数解析、アプリ初期化、終了コードの返却
- `src/app.rs`
  - アプリ全体の制御、メインループ、停止条件の管理
- `src/renderer.rs`
  - 画面描画（ターミナルへの描画命令の生成と出力）
- `src/actor.rs`
  - 動物の抽象インターフェース（例: 位置更新、描画用スプライト）
- `src/rabbit.rs`
  - ウサギ固有の挙動、スプライト定義、移動ロジック
- `src/frame.rs`
  - フレームごとの描画内容（バッファ）と差分更新の表現
- `src/time.rs`
  - フレームレート制御、スリープ、経過時間計測
- `src/error.rs`
  - エラー型の集約と表示方針

## 3. 主要な構造体・enum
- `AppConfig`
  - 役割: 実行設定の保持
  - フィールド例: `fps: u32`, `width: u16`, `height: u16`, `animal: AnimalKind`
- `App`
  - 役割: アプリ全体の状態とループ制御
  - フィールド例: `config: AppConfig`, `renderer: Renderer`, `actor: Box<dyn Actor>`, `clock: Clock`
- `Renderer`
  - 役割: 画面描画、フレームバッファの表示
  - フィールド例: `last_frame: Frame`, `size: (u16, u16)`
- `Frame`
  - 役割: 描画用バッファ、差分更新の元データ
  - フィールド例: `cells: Vec<Cell>`, `size: (u16, u16)`
- `Cell`
  - 役割: 1セル分の描画情報
  - フィールド例: `ch: char`, `style: CellStyle`
- `Clock`
  - 役割: フレーム間隔の制御
  - フィールド例: `frame_duration: Duration`, `last_tick: Instant`
- `Actor`（trait）
  - 役割: 動物の共通インターフェース
  - メソッド例: `update(dt)`, `render(&mut Frame)`
- `Rabbit`
  - 役割: ウサギの状態と挙動
  - フィールド例: `pos: (i16, i16)`, `velocity: (i16, i16)`, `sprite: Sprite`
- `Sprite`
  - 役割: 複数フレームのアスキーアート保持
  - フィールド例: `frames: Vec<Vec<String>>`, `current: usize`
- `AnimalKind`
  - 役割: 選択可能な動物種別
  - 例: `Rabbit`（将来: `Cat`, `Dog` など）

## 4. 主要な処理フロー
- 起動
  - CLI 引数を解析し `AppConfig` を作成
  - `Renderer` と `Actor` を初期化
  - `App` を生成してメインループに移行
- メインループ
  - `Clock` から経過時間を取得
  - `Actor::update` で位置やアニメーションを更新
  - `Frame` をクリアして `Actor::render` を実行
  - `Renderer` が `Frame` をターミナルへ描画
  - 次フレームまでスリープ
- 終了
  - 正常終了で描画を停止
  - 必要に応じてカーソル表示復帰などを行う

## 5. 使用を検討すべきクレート
- `clap`
  - CLI 引数解析を簡潔に保つ
- `crossterm`
  - ターミナルの描画、カーソル操作、サイズ取得
- `anyhow` または `thiserror`
  - エラーの集約と表示を簡潔にする

## 6. テストしやすくするための設計ポイント
- 描画ロジックと状態更新ロジックを分離し、`Actor::update` を純粋関数に近づける
- `Renderer` は I/O を持つため、`Frame` の生成を別モジュールで行う
- `Clock` を差し替え可能にして、テスト時は擬似時間を注入できるようにする
- `Actor` を trait で抽象化し、テスト用のダミー実装を用意しやすくする
- `AppConfig` は単体で検証できるようにし、引数解析と分離する
