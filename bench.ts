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
    "Please check the system settings."  // TM Index 1
];

const tb = [
    "document", // TB Index 0
    "system",   // TB Index 1
    "updated"   // TB Index 2
];

const texts = [
    "The document is updated.",          // Text 0: TM[0]と類似、TB[0,2]を含む
    "Check system settings please.",      // Text 1: TM[1]と類似、TB[1]を含む
    "The document is updated."           // Text 2: Text[0]と重複、TB[0,2]を含む
];

console.log("--- Rust Analysis Engine Test ---");
const startTime = performance.now();

// Rustエンジンの実行（しきい値 0.6）
const results: AnalysisResult[] = analyze_all(tm, texts, tb, 0.6);

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

    // ファイル内重複（前方一致）の表示
    if (res.i.length > 0) {
        res.i.forEach(idx => {
            console.log(`  -> ⚠️ 前方一致 [Text ${idx}]: "${texts[idx]}"`);
        });
    }

    // 用語ヒットの表示
    if (res.g.length > 0) {
        const hits = res.g.map(idx => tb[idx]).join(", ");
        console.log(`  -> 📘 用語検出: ${hits}`);
    }
});