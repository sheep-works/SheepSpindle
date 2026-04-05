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
import { search_hybrid } from './pkg/rust_tm_engine';
```

または

```js
const { find_similar_pairs } = require('./pkg/rust_tm_engine');
```

## 実行

```bash
npx ts-node-esm bench.ts
```