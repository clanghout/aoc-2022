let OpponentMove = {
    A: 1, // rock
    B: 2, // paper
    C: 3, // Scissors
}

let YourMove = {
    X: 1, // R
    Y: 2, // P
    Z: 3, // S
}

async function main() {
    const test = 'inputs/day2test.txt'
    const input = 'inputs/input2.txt'

    async function computePart1(inputFilePath: any): Promise<number> {
        const file = await Deno.readTextFile(inputFilePath);
        let counter = 0;
        file.trim().split("\n").map(line => {
            let [om, mm] = line.trim().split(" ")
            let omn: number = OpponentMove[om]
            let mmn: number = YourMove[mm]
            counter += mmn;
            if (omn === mmn) {
                counter += 3
            } else if (omn === mmn - 1 || (omn === 3 && mmn === 1)) { // win
                counter += 6
            }
        })
        return counter
    }

    async function computePart2(inputFilePath: any): Promise<number> {
        const readStream = await Deno.readTextFile(inputFilePath);
        let counter = 0;
        readStream.trim().split("\n").map(line => {
            let [om, mm] = line.trim().split(" ")
            let omn: number = OpponentMove[om]
            let mmn = mm === "X" ? //lose
                omn - 1 :
                mm === "Y" ? //draw
                    omn :
                    omn + 1; // win;
            // roll over
            mmn = mmn === 0 ? 3 : mmn;
            mmn = mmn === 4 ? 1 : mmn;

            counter += mmn
            if (omn === mmn) {
                counter += 3
            } else if (omn === mmn - 1 || (omn === 3 && mmn === 1)) { // win
                counter += 6
            }
        })
        return counter
    }

    console.log('Part 1')
    console.log('test: ')
    console.log(await computePart1(test))
    console.log('Expecting 15')
    console.log('\nInput:')
    console.log(await computePart1(input))
    console.log('\n\nPart 2')
    console.log('test: ')
    console.log(await computePart2(test))
    console.log('Expecting 12')
    console.log('\nInput:')
    console.log(await computePart2(input))
}

main();