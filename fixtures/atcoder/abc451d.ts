// ================================================================
// AtCoder Beginner Contest 451
// D - Concat Power of 2
// (URL: https://atcoder.jp/contests/abc451/tasks/abc451_d)
// TypeScript (Bun) Submission
// code from https://github.com/AXT-AyaKoto/perry-ts-test-2026-0421/blob/main/abc451d-bun.ts
// ================================================================

function search(before: string, powersOfTwoStr: string[]): string[] {
    const answers: string[] = [];
    if (before.length > 0) answers.push(before); // 当然ですがbefore自身は回答になりますからね？
    const remainDigits = 9 - before.length;
    for (let i = 0; i < powersOfTwoStr.length; i++) {
        const after = powersOfTwoStr[i];
        if (after.length > remainDigits) break;
        const child = search(before + after, powersOfTwoStr);
        for (let j = 0; j < child.length; j++) {
            answers.push(child[j]);
        }
    }
    return answers;
};

function Main(inputText: string): void {
    const input: string[][] = inputText.trim().split("\n").map(row => row.split(" "));
    // Add your code here
    const N: number = +input[0][0];
    // 10^9は10桁で、10^9自体は良い整数じゃないので9桁まで考えればOK
    // とりあえず、1〜9桁の2^nを全部列挙します
    const powersOfTwo: number[] = [];
    for (let i = 0; 2 ** i <= 1e9; i++) {
        powersOfTwo.push(2 ** i);
    }
    const powersOfTwoStr: string[] = powersOfTwo.map(n => String(n));
    // DFSっぽいことをすればいけるんじゃないか？
    /** beforeでここまでの結合例を持ってくるので、9桁以内の全結合パターン列挙してstringで戻す */
    const allGoodIntStr = search("", powersOfTwoStr);
    const allGoodIntHasDup = allGoodIntStr.map(n => +n);
    const allGoodIntSet = new Set<number>();
    for (let i = 0; i < allGoodIntHasDup.length; i++) {
        allGoodIntSet.add(allGoodIntHasDup[i]);
    }
    const allGoodInt = [...allGoodIntSet];
    allGoodInt.sort((a, b) => a - b);
    console.log(allGoodInt[N - 1]);
    return;
}

// Please do not change the following code.
export { }; // <- An empty export is required so that ts-check can determine it as a module.
// @ts-ignore
Main(await Bun.file("/dev/stdin").text());