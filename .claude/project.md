# sid-convert プロジェクト

## 📋 プロジェクト概要

**sid-convert** は、MIDIファイルからベースパートを自動抽出し、人間が読みやすいYAML形式で出力するRust製CLIツールです。

### クイックスタート

```bash
# ビルド
cargo build --release

# 実行
./target/release/sid-convert input.mid

# トラック指定
./target/release/sid-convert input.mid --track 1 --output my-bass.yaml
```

## 🎯 主要機能

1. **自動ベース検出**: トラック名または音域からベースパートを自動検出
2. **音名変換**: MIDIノート番号 → 音名（例: 40 → E2）
3. **音価変換**: Tick数 → 音価（quarter/eighth/half/whole）
4. **YAML出力**: 人間可読で拡張性の高いフォーマット

## 📁 ドキュメント構成

### メインドキュメント
- **[README.md](../README.md)**: ユーザー向け使用ガイド

### 詳細ドキュメント (.claude/docs/)
- **[概要.md](docs/概要.md)**: プロジェクトのビジョン、目的、スコープ
- **[設計.md](docs/設計.md)**: アーキテクチャ、モジュール設計、技術仕様
- **[進捗.md](docs/進捗.md)**: 開発履歴、マイルストーン、完成度
- **[仕様決定履歴.md](docs/仕様決定履歴.md)**: 全ての設計判断と理由

## 🏗️ プロジェクト構造

```
sid-convert/
├── Cargo.toml              # Rust依存関係
├── README.md               # ユーザーガイド
├── .gitignore
├── src/
│   ├── main.rs            # CLIエントリーポイント (106行)
│   ├── midi.rs            # MIDI解析・ベース検出 (150行)
│   ├── convert.rs         # ノート変換ロジック (85行)
│   └── export.rs          # YAML出力 (75行)
└── .claude/
    ├── project.md         # このファイル
    └── docs/
        ├── 概要.md        # プロジェクト概要
        ├── 設計.md        # 技術設計
        ├── 進捗.md        # 開発進捗
        └── 仕様決定履歴.md # 仕様決定の記録
```

## 🔧 技術スタック

| カテゴリ | 技術 | バージョン | 用途 |
|---------|------|-----------|------|
| 言語 | Rust | 2021 edition | 安全性・パフォーマンス |
| MIDI解析 | midly | 0.5 | MIDIファイル解析 |
| CLI | clap | 4.5 | コマンドライン引数 |
| シリアライズ | serde | 1.0 | データ構造変換 |
| YAML | serde_yaml | 0.9 | YAML出力 |

## 📊 開発ステータス

| 項目 | ステータス | 備考 |
|-----|----------|------|
| **実装** | ✅ 完成 | 全機能実装済み |
| **テスト** | ✅ 完成 | 5テスト全パス |
| **ドキュメント** | ✅ 完成 | 包括的ドキュメント |
| **ビルド** | ✅ 成功 | debug/release両対応 |
| **デプロイ** | ✅ 完了 | ブランチにプッシュ済み |

**最終更新**: 2025年11月17日
**ブランチ**: `claude/sid-convert-midi-parser-01EonDWa8YWmHKJNzZH1JDrU`
**最新コミット**: `597a1ac`

## 🎵 出力例

### 入力: MIDIファイル
```
Track 0: Drums
Track 1: Bass (45 notes in bass range)
Track 2: Piano
```

### 出力: YAML (sid-convert.yaml)
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

## 🎯 設計思想

### 核心原則
1. **シンプルさ**: 複雑さを避け、実用的な機能に集中
2. **ユーザー中心**: ベース演奏者のニーズを最優先
3. **拡張性**: 将来の機能追加を妨げない設計
4. **品質**: テスト・ドキュメント完備

### スコープ判断

✅ **含む**:
- ベーストラック自動検出
- 音名・音価の変換
- 人間可読なYAML出力

❌ **含まない** (意図的に除外):
- テンポ変化の追跡（ベースには不要）
- 転調の検出（ベースには影響少）
- フレット計算（sid-noteが担当）
- 細かい音価（16分音符等）

## 🧪 テスト

### ユニットテスト (5テスト)
```bash
cargo test
```

**カバレッジ**:
- ✅ `test_note_to_pitch`: 音名変換
- ✅ `test_ticks_to_duration`: 音価変換
- ✅ `test_is_bass_range`: ベース音域判定
- ✅ `test_output_creation`: YAML構造作成
- ✅ `test_to_yaml`: YAML出力検証

### 実行例
```bash
$ cargo test
running 5 tests
test convert::tests::test_is_bass_range ... ok
test convert::tests::test_note_to_pitch ... ok
test convert::tests::test_ticks_to_duration ... ok
test export::tests::test_output_creation ... ok
test export::tests::test_to_yaml ... ok

test result: ok. 5 passed; 0 failed
```

## 🚀 使用例

### 基本的な使い方
```bash
# 自動検出（ベーストラックが1つの場合）
sid-convert song.mid

# 複数トラックから選択
sid-convert song.mid
# → 候補が表示される
# Multiple bass tracks found. Please specify one with --track <number>:
#   Track 1: Bass (42/50 bass notes)
#   Track 2: Bass Line (38/40 bass notes)

sid-convert song.mid --track 1

# 出力ファイル指定
sid-convert song.mid --output my-bass.yaml
```

## 📈 今後の拡張可能性

### 優先度: 高
- [ ] サンプルMIDIファイルでの統合テスト
- [ ] CI/CD設定（GitHub Actions）
- [ ] バイナリリリース（GitHub Releases）

### 優先度: 中
- [ ] ベロシティ情報の出力
- [ ] 複数トラック一括処理
- [ ] JSON形式対応
- [ ] エラーメッセージの多言語化

### 優先度: 低
- [ ] GUI版の開発
- [ ] リアルタイムMIDI入力
- [ ] WebAssembly対応

## 🤝 開発への参加

### ビルド
```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release
```

### テスト
```bash
# 全テスト実行
cargo test

# 特定のテスト
cargo test test_note_to_pitch
```

### ドキュメント
- ユーザー向け: [README.md](../README.md)
- 開発者向け: [.claude/docs/](docs/)

## 📝 重要な技術的決定

### ライフタイム管理
```rust
// Box::leak() を使用して 'static 参照を作成
// CLIツールは短命なのでメモリリークは許容
let data_static: &'static [u8] = Box::leak(data.clone().into_boxed_slice());
```

### ベース検出ロジック
```rust
// 条件1: トラック名
is_bass_by_name = track_name.to_lowercase().contains("bass")

// 条件2: 音域（70%以上がE1～G3）
is_bass_by_range = (bass_notes / total_notes) > 0.7
```

### 音価の丸め処理
```rust
// 人間の演奏のゆらぎを考慮した許容誤差
if (quarters - 1.0).abs() < 0.25 { "quarter" }
else if (quarters - 0.5).abs() < 0.125 { "eighth" }
```

詳細は [仕様決定履歴.md](docs/仕様決定履歴.md) を参照

## 🐛 トラブルシューティング

### Q: "No bass tracks found"と表示される
**A**: MIDIファイルにベーストラックが含まれていない可能性があります。
- `--track`オプションで手動指定を試してください
- トラック名に"Bass"が含まれているか確認

### Q: 音価が意図と異なる
**A**: MIDI演奏のタイミングにゆらぎがある可能性があります。
- クォンタイズされたMIDIファイルの使用を推奨
- 許容誤差は±25%（仕様）

### Q: ビルドエラーが出る
**A**: Rustのバージョンを確認してください。
```bash
rustc --version  # 1.70以上推奨
cargo --version
```

## 📄 ライセンス

MIT License

---

**開発者**: Claude Code
**プロジェクト開始**: 2025年11月17日
**初回リリース**: v0.1.0 (2025年11月17日)

詳細情報は各ドキュメントを参照してください。
