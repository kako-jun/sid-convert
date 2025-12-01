# sid-convert 開発者向けドキュメント

MIDIファイルからベースパートを抽出し、YAML形式で出力するRust製CLIツール。

## プロジェクト構造

```
src/
├── main.rs      # CLIエントリーポイント、引数解析
├── midi.rs      # MIDI読み込み、ベーストラック検出
├── convert.rs   # ノート番号→音名、tick→音価の変換
└── export.rs    # YAML出力
```

## 依存クレート

- `midly` 0.5: MIDI解析
- `clap` 4.5: CLI引数パーサー
- `serde` 1.0 + `serde_yaml` 0.9: YAML出力

## ビルド・テスト

```bash
cargo build           # デバッグビルド
cargo build --release # リリースビルド
cargo test            # テスト実行
```

## アーキテクチャ

```
MIDIファイル → MidiParser.from_file()
            → find_bass_tracks()  # ベーストラック候補を検出
            → extract_notes()     # 指定トラックのノート抽出
            → Output.save_to_file() # YAML出力
```

## 主要な設計判断

### ベーストラック検出

2つの条件のいずれかを満たせばベーストラックと判定：

1. トラック名に「bass」を含む（大文字小文字無視）
2. ベース音域（E1〜G3）のノートが70%以上

### 音価変換

tick数から音価への変換は許容誤差を設けて丸め処理：

- whole: 4.0 ± 0.25拍
- half: 2.0 ± 0.25拍
- quarter: 1.0 ± 0.25拍
- eighth: 0.5 ± 0.125拍

### ライフタイム管理

`midly`が`'static`参照を要求するため、`Box::leak()`を使用：

```rust
let data_static: &'static [u8] = Box::leak(data.clone().into_boxed_slice());
```

CLIツールは短命なのでメモリリークは許容。

## 仕様

### ベース音域

- 下限: E1（MIDIノート28）= 4弦ベースの最低音
- 上限: G3（MIDIノート55）= ベースの実用的な高音域上限

### 意図的に除外した機能

- テンポ変化の追跡（ベース演奏には重要度が低い）
- 転調の検出（ベースには影響が少ない）
- 16分音符以下の細かい音価（ベースでは稀）
- フレット計算（sid-noteが担当）
