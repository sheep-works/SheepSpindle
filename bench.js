const { bench_wasm } = require('./pkg/rust_tm_engine');

const SIZE = 10000;

console.log(`Generating ${SIZE} lines of test data in JS...`);
const testData = Array.from({ length: SIZE }, () =>
    Math.random().toString(36).substring(2, 12) // 10文字のランダム文字列
);

// --- JS版の処理計測 ---
console.log("Starting JS Benchmark...");
console.time("Pure JS Time");
let jsCount = 0;
for (let i = 0; i < testData.length; i++) {
    for (let j = 0; j < i; j++) {
        if (testData[i].includes(testData[j]) || testData[j].includes(testData[i])) {
            jsCount++;
            break;
        }
    }
}
console.timeEnd("Pure JS Time");

// --- Rust Wasm版の処理計測 ---
console.log("Starting Rust Wasm Benchmark...");
console.time("Rust Wasm Time");
const wasmCount = bench_wasm(testData);
console.timeEnd("Rust Wasm Time");

console.log(`Results match: ${jsCount === wasmCount} (Count: ${jsCount})`);