import { analyze_text } from './pkg/rust_tm_engine.js';

// Rustから返ってくるデータの型定義
interface AnalysisResult {
    tm_indices: number[];
    prev_indices: number[];
    tb_indices: number[];
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
const results: AnalysisResult[] = analyze_text(tm, texts, tb, 0.6);

const endTime = performance.now();
console.log(`Processing Time: ${(endTime - startTime).toFixed(3)}ms\n`);

// 結果の表示
results.forEach((res, i) => {
    console.log(`[Text ${i}] "${texts[i]}"`);

    // TM一致の表示
    if (res.tm_indices.length > 0) {
        res.tm_indices.forEach(idx => {
            console.log(`  -> 💡 TMマッチ [${idx}]: "${tm[idx]}"`);
        });
    }

    // ファイル内重複（前方一致）の表示
    if (res.prev_indices.length > 0) {
        res.prev_indices.forEach(idx => {
            console.log(`  -> ⚠️ 前方一致 [Text ${idx}]: "${texts[idx]}"`);
        });
    }

    // 用語ヒットの表示
    if (res.tb_indices.length > 0) {
        const hits = res.tb_indices.map(idx => tb[idx]).join(", ");
        console.log(`  -> 📘 用語検出: ${hits}`);
    }
});