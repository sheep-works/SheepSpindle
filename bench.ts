import { analyze_all } from './pkg/sheep_spindle.js';

// Rustから返ってくるデータの型定義
interface AnalysisResult {
    t: number[]; // Translation Memory
    i: number[]; // Internal duplication
    g: number[]; // Glossary
}

// 実際のデータを模したテストセット
const tm = [
    "The document is being updated.",    // TM Index 0
    "Please check the system settings.", // TM Index 1
    "1000000 apples",
    "1000001 apples",
    "1000002 apples",
    "1000003 apples",
    "1000004 apples",
    "1000005 apples",
    "1000006 apples",
    "1000007 apples",
    "1000008 apples",
    "1000009 apples",
    "1000010 apples",
    "1000011 apples",
    "1000012 apples",
    "10000000 apples"
];

const tb = [
    "document", // TB Index 0
    "system",   // TB Index 1
    "updated",   // TB Index 2
    "0 apples"
];

const texts = [
    "The document is updated.",          // Text 0: TM[0]と類似、TB[0,2]を含む
    "Check system settings please.",      // Text 1: TM[1]と類似、TB[1]を含む
    "The document is updated.",           // Text 2: Text[0]と重複、TB[0,2]を含む
    "1000000 apples",
    "10000000 apples",
    "100000000 apples",
];

console.log("--- Rust Analysis Engine Test ---");
const startTime = performance.now();

// Rustエンジンの実行（しきい値 0.6）
const results: AnalysisResult[] = analyze_all(tm, texts, tb, 0.6, 5);

const endTime = performance.now();
console.log(`Processing Time: ${(endTime - startTime).toFixed(3)}ms\n`);

// 結果の表示
results.forEach((res, i) => {
    console.log(`[Text ${i}] "${texts[i]}"`);

    // TM一致の表示
    if (res.t.length > 0) {
        res.t.forEach(idx => {
            console.log(`  -> 💡 TMマッチ [${idx}]: "${tm[idx]}"`);
        });
    }

    // ファイル内重複（内部マッチ）の表示
    if (res.i.length > 0) {
        res.i.forEach(idx => {
            console.log(`  -> ⚠️ 内部マッチ [Text ${idx}]: "${texts[idx]}"`);
        });
    }

    // 用語ヒットの表示
    if (res.g.length > 0) {
        const hits = res.g.map(idx => tb[idx]).join(", ");
        console.log(`  -> 📘 用語検出: ${hits}`);
    }
});