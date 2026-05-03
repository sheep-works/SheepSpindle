# SheepSpindle

翻訳作業で欠かせない TM/TB の適用や全体 QA 作業を Rust ＆ WASM の力で高速化するプロジェクトです。

## 主な機能

### 1. 類似度検索 (TM Search)
大量の翻訳メモリ (TM) から類似する原文を高速に抽出します。

### 2. 用語抽出 (TB Search)
Aho-Corasick アルゴリズムを使用し、原文中から用語集 (TB) に登録された用語を高速に抽出します。

### 3. 一括解析 (analyze_all)
TM、TB、およびファイル内の前方一致（重複）を一度に解析します。

### 4. 訳文のゆれチェック (get_consistency_groups)
原文が同じ、または類似しているのに訳文が異なる箇所をグループ化します。

### 5. CSVへのTM適用 (add_tm_to_csv)
CSV形式の対訳データに対し、TMから類似文を自動で付与します。差分を `[INS]`, `[REPLACE]` タグで可視化する機能を含みます。

## Rust ファイルのビルド

### Node.js環境で実行
```bash
wasm-pack build --target nodejs
```

### Web環境 (ブラウザ) で実行
```bash
wasm-pack build --target web --out-dir pkg-web
```

## 公開関数 (WASM API)

### `add_tm_to_csv(csv, tm, config)`
CSVデータにTMマッチング結果を追加します。
- **csv**: `{ 行番号: string, 原文: string, 訳文: string, 備考: string }` の配列
- **tm**: `{ src: string, tgt: string }` の配列
- **config**:
    - `threshold`: 類似度しきい値 (0.0 ~ 100.0)
    - `pruning_pct`: 前処理での長さ枝切り率 (0.0 ~ 1.0)
    - `max_matches`: 取得する最大マッチ数

### `analyze_all(tm, texts, tb, threshold, counts)`
全ての解析（TM/TB/重複）を同時に行います。
- **threshold**: 0.0 ~ 1.0 の範囲で指定

## 注意事項

数字と記号のみからなる文字列は、誤判定防止のため類似度比較の対象外となります。

```rs
// 判定対象外となる文字列の例
s.chars().all(|c| {
    matches!(c,
        '0'..='9' | '０'..='９' |
        '(' | ')' | '（' | '）' | '【' | '】' | '[' | ']' |
        '%' | '％' |
        '.' | ',' | ':' | ';' | '/' | '+' | '±' |
        ' ' | '　' |
        // ... (詳細は src/lib.rs を参照)
    )
})
```