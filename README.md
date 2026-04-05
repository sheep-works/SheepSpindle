# SheepSpindle

翻訳作業で欠かせない TM/TB の適用や全体 QA 作業を Rust ＆ WASM の力で高速化するプロジェクトです

# Rust ファイルのビルド

## 対象

- src/lib.rs

## コマンド

```bash
wasm-pack build --target nodejs
```

# 実行

## インポート

```ts
// pkgの中に生成されたJSをインポート
import { search_hybrid } from './pkg/sheep_spindle';
```

または

```js
const { find_similar_pairs } = require('./pkg/sheep_spindle');
```

## 実行

```bash
npx ts-node-esm bench.ts
```