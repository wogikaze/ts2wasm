function search(before: string, powersOfTwoStr: string[]): string[] {
    const answers: string[] = [];
    if (before.length > 0) answers.push(before);
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
}

function Main(inputText: string): void {
    const input: string[][] = inputText.trim().split("\n").map(row => row.split(" "));
    const N: number = +input[0][0];
    const powersOfTwo: number[] = [];
    for (let i = 0; 2 ** i <= 1000000000; i++) {
        powersOfTwo.push(2 ** i);
    }
    const powersOfTwoStr: string[] = powersOfTwo.map(n => String(n));
    const allGoodIntStr = search("", powersOfTwoStr);
    const allGoodIntHasDup = allGoodIntStr.map(n => +n);
    const allGoodIntSet = new Set<number>();
    for (let i = 0; i < allGoodIntHasDup.length; i++) {
        allGoodIntSet.add(allGoodIntHasDup[i]);
    }
    const allGoodInt = [...allGoodIntSet];
    allGoodInt.sort((a, b) => a - b);
    console.log(allGoodInt[N - 1]);
}

Main(require("fs").readFileSync(0, "utf8"));
