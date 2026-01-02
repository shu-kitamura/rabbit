# テスト設計: CLI Rabbit

## 1. テスト対象の整理
- `Frame` / `Cell` の基本操作（生成、取得、設定、クリア）
- `Sprite` のフレーム管理（空配列の拒否、フレーム循環）
- `Rabbit` の状態更新と描画（更新での位置変化、描画結果）
- `Clock` のフレーム間隔計算（fps からの Duration 生成）
- 例外や境界チェック（範囲外アクセスのエラー）
- テストしない範囲
  - `Renderer` の実ターミナル出力
  - 実時間の `sleep` 動作
  - `clap` による CLI 解析の詳細

## 2. テスト観点の洗い出し
- 正常系
  - 生成した `Frame` のサイズ・初期値が正しい
  - `Sprite` がフレームを循環する
  - `Rabbit` が更新・描画で期待どおりの結果を返す
  - `Clock` の `frame_duration` が fps に応じて決まる
- 異常系
  - `Frame` の範囲外アクセスが `Err` になる
  - `Sprite` の空フレーム生成が `Err` になる
- 境界値・エッジケース
  - `Frame` のサイズが 1x1 の場合
  - `Rabbit` の速度が 0 の場合
  - `Sprite` の最終フレームからの wrap
- 状態遷移
  - `Rabbit` 更新の連続適用
  - `Sprite` の連続 `advance` による循環

## 3. テストケース一覧
- `frame_new_sets_size_and_default`
  - 目的: `Frame` 初期化時のサイズと初期セルを確認する
  - 入力: `Frame::new(3, 2)`
  - 期待: `size == (3, 2)`、全セルがデフォルト
- `frame_set_get_in_bounds`
  - 目的: 範囲内セルの設定と取得が正しく動作する
  - 入力: `(1, 1)` に `Cell` を設定
  - 期待: 取得したセルが一致する
- `frame_set_out_of_bounds_returns_err`
  - 目的: 範囲外アクセスの扱いを明示する
  - 入力: `(10, 10)` に設定
  - 期待: `Err` を返す
- `frame_clear_resets_cells`
  - 目的: `clear` で全セルが初期化される
  - 入力: 任意セルを書き換えた後に `clear`
  - 期待: 全セルがデフォルト
- `sprite_new_rejects_empty_frames`
  - 目的: 空フレームを拒否できること
  - 入力: `frames = vec![]`
  - 期待: `Err` を返す
- `sprite_advance_wraps`
  - 目的: フレームが循環すること
  - 入力: 2 フレームの `Sprite` で `advance` を 2 回
  - 期待: `current` が初期フレームに戻る
- `rabbit_update_zero_velocity_no_move`
  - 目的: 速度 0 の場合は位置が変わらない
  - 入力: `velocity = (0, 0)`、`dt = 1s`
  - 期待: `pos` が更新前と同一
- `rabbit_render_draws_sprite_at_position`
  - 目的: スプライトが指定位置に描画される
  - 入力: `pos = (0, 0)`、`sprite = ["R"]`
  - 期待: `Frame` の `(0, 0)` が `R`
- `clock_frame_duration_from_fps`
  - 目的: fps から正しいフレーム間隔が導出される
  - 入力: `fps = 60`
  - 期待: `frame_duration` が `1/60s` 付近

## 4. 実装エージェントへの注意点
- `Frame` の座標系は `(x, y)` を `(col, row)` とし、原点は左上で統一する
- `Frame` の範囲外アクセスは必ず `Err` で検知できるようにする
- `Sprite` は空配列を許可しないこと（`Err` で返す）
- `Rabbit::update` は `Duration` を受け、速度 0 のときは必ず位置が不変
- `Clock` の計算は純粋関数に近づけ、`fps` からの変換を単体で検証できるようにする
- テストでは実ターミナル I/O を触らない前提なので、描画結果は `Frame` に残す

```rust
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::frame::{Cell, Frame};
    use crate::rabbit::{Rabbit, Sprite};
    use crate::time::Clock;

    #[test]
    fn frame_new_sets_size_and_default() {
        let frame = Frame::new(3, 2);
        assert_eq!(frame.size(), (3, 2));
        for y in 0..2 {
            for x in 0..3 {
                assert_eq!(frame.get(x, y).unwrap(), &Cell::default());
            }
        }
    }

    #[test]
    fn frame_set_get_in_bounds() {
        let mut frame = Frame::new(2, 2);
        let cell = Cell::from_char('R');
        frame.set(1, 1, cell.clone()).unwrap();
        assert_eq!(frame.get(1, 1).unwrap(), &cell);
    }

    #[test]
    fn frame_set_out_of_bounds_returns_err() {
        let mut frame = Frame::new(2, 2);
        let cell = Cell::from_char('R');
        assert!(frame.set(10, 10, cell).is_err());
    }

    #[test]
    fn frame_clear_resets_cells() {
        let mut frame = Frame::new(2, 2);
        frame.set(0, 0, Cell::from_char('R')).unwrap();
        frame.clear();
        assert_eq!(frame.get(0, 0).unwrap(), &Cell::default());
    }

    #[test]
    fn sprite_new_rejects_empty_frames() {
        let frames: Vec<Vec<String>> = vec![];
        assert!(Sprite::new(frames).is_err());
    }

    #[test]
    fn sprite_advance_wraps() {
        let frames = vec![
            vec!["R".to_string()],
            vec!["r".to_string()],
        ];
        let mut sprite = Sprite::new(frames).unwrap();
        let first = sprite.current_frame().to_vec();
        sprite.advance();
        sprite.advance();
        assert_eq!(sprite.current_frame(), first.as_slice());
    }

    #[test]
    fn rabbit_update_zero_velocity_no_move() {
        let frames = vec![vec!["R".to_string()]];
        let sprite = Sprite::new(frames).unwrap();
        let mut rabbit = Rabbit::new((1, 1), (0, 0), sprite);
        rabbit.update(Duration::from_secs(1));
        assert_eq!(rabbit.position(), (1, 1));
    }

    #[test]
    fn rabbit_render_draws_sprite_at_position() {
        let frames = vec![vec!["R".to_string()]];
        let sprite = Sprite::new(frames).unwrap();
        let rabbit = Rabbit::new((0, 0), (0, 0), sprite);
        let mut frame = Frame::new(1, 1);
        rabbit.render(&mut frame);
        assert_eq!(frame.get(0, 0).unwrap(), &Cell::from_char('R'));
    }

    #[test]
    fn clock_frame_duration_from_fps() {
        let clock = Clock::from_fps(60).unwrap();
        let dur = clock.frame_duration();
        assert_eq!(dur, Duration::from_nanos(1_000_000_000 / 60));
    }
}
```
