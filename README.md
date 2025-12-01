# sid-convert

MIDIファイルからベースパートを抽出し、YAML形式で出力するCLIツール。

## インストール

```bash
cargo build --release
```

## 使い方

```bash
# 基本（ベーストラックを自動検出）
sid-convert song.mid

# トラックを指定
sid-convert song.mid --track 1

# 出力ファイル名を指定
sid-convert song.mid --output my-bass.yaml
```

### 複数のベーストラックがある場合

自動検出で複数のベーストラックが見つかると、候補が表示されます：

```
Multiple bass tracks found. Please specify one with --track <number>:
  Track 1: Bass (42/50 bass notes)
  Track 2: Bass Line (38/40 bass notes)
```

`--track`オプションでトラック番号を指定してください。

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
```

- `start`: 開始位置（tick）
- `length`: 音価（whole / half / quarter / eighth）
- `pitch`: 音名（例: E2, F#2）

## オプション一覧

```
Arguments:
  <FILE>  入力MIDIファイル

Options:
  -t, --track <TRACK>    抽出するトラック番号 (0始まり)
  -o, --output <OUTPUT>  出力ファイル [default: sid-convert.yaml]
  -h, --help             ヘルプ
  -V, --version          バージョン
```

## ライセンス

MIT
