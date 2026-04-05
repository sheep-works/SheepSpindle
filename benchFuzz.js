const { find_similar_pairs } = require('./pkg/rust_tm_engine');

const testData = [
    "Rust is very fast",      // 0
    "Rust is extremely fast", // 1 (0と似ている)
    "TypeScript is great",    // 2
    "Rust is fast",           // 3 (0, 1と似ている)
];

console.log("Starting Precise Similarity Check...");
const threshold = 0.5; // 70% 以上の類似度
const similarMap = find_similar_pairs(testData, threshold);

similarMap.forEach((matches, i) => {
    if (matches.length > 0) {
        console.log(`Entry [${i}] "${testData[i]}"`);
        matches.forEach(matchIdx => {
            console.log(`  -> Similar to [${matchIdx}] "${testData[matchIdx]}"`);
        });
    }
});