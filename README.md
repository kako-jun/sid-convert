# sid-convert

MIDIファイルからベースパートを抽出し、YAML形式で出力するRust製のCLIツール。

## 概要

`sid-convert` は、MIDIファイルからベーストラックを自動検出・抽出し、音程と音価を人間が読みやすいYAML形式で出力します。sid-note側でのフレット変換を前提としたデータ形式を採用しています。

## 機能

- **自動ベーストラック検出**
  - トラック名に「Bass」を含むトラックを検出
  - または音域が低音（E1～G3）のトラックを検出
- **音価変換**
  - MIDIのtick数から quarter, eighth, half, whole に変換
  - 4/4拍子を前提（ベース演奏に適した設定）
- **YAML出力**
  - 人間可読な形式
  - 拡張性が高く、将来的に運指情報などを追加可能

## インストール

```bash
cargo build --release
```

ビルドされたバイナリは `target/release/sid-convert` にあります。

## 使い方

### 基本的な使用方法

```bash
sid-convert input.mid
```

### オプション

```bash
sid-convert [OPTIONS] <FILE>

Arguments:
  <FILE>  入力MIDIファイルパス

Options:
  -t, --track <TRACK>    抽出するトラック番号 (0始まり)
  -o, --output <OUTPUT>  出力YAMLファイルパス [default: sid-convert.yaml]
  -h, --help             ヘルプを表示
  -V, --version          バージョンを表示
```

### 例

#### 1. 自動検出（ベーストラックが1つの場合）

```bash
sid-convert song.mid
```

出力: `sid-convert.yaml`

#### 2. 複数のベーストラックがある場合

```bash
sid-convert song.mid
```

出力例:
```
Multiple bass tracks found. Please specify one with --track <number>:
  Track 1: Bass (42/50 bass notes)
  Track 2: Bass Line (38/40 bass notes)
```

トラックを指定して実行:
```bash
sid-convert song.mid --track 1
```

#### 3. 出力ファイル名を指定

```bash
sid-convert song.mid --output my-bass.yaml
```

## 出力形式

```yaml
track: "Bass"
notes:
  - start: 0
    length: quarter
    pitch: E2
  - start: 480
    length: eighth
    pitch: F#2
  - start: 720
    length: quarter
    pitch: G2
```

### フィールド説明

- `track`: トラック名
- `notes`: ノートのリスト
  - `start`: 開始位置（tick数）
  - `length`: 音価（quarter/eighth/half/whole）
  - `pitch`: 音程（例: E2, F#2）

## 仕様

### ベース音域

E1（28）～ G3（55）のMIDIノート番号

### 音価

- `whole`: 全音符（4拍）
- `half`: 2分音符（2拍）
- `quarter`: 4分音符（1拍）
- `eighth`: 8分音符（0.5拍）

### 前提条件

- 拍子: 4/4（固定）
- テンポ変化: 無視
- 転調: 無視（ベース演奏には影響が少ないため）

## 技術仕様

### 依存クレート

- `midly` (0.5): MIDI解析
- `serde` (1.0): シリアライゼーション
- `serde_yaml` (0.9): YAML出力
- `clap` (4.5): CLIパーサー

### モジュール構成

- `src/main.rs`: CLIエントリーポイント
- `src/midi.rs`: MIDI読み込み＆Bassトラック抽出
- `src/convert.rs`: ノート番号→音名、音価変換
- `src/export.rs`: YAML出力

## 開発

### テスト実行

```bash
cargo test
```

### ビルド（デバッグ）

```bash
cargo build
```

### ビルド（リリース）

```bash
cargo build --release
```

## ライセンス

MIT
