export declare class Bitwidth {
    #private;
    constructor(bits: number);
    get bits(): number;
    add(a: number, b: number): number;
    complement(a: number): number;
    subtract(a: number, b: number): number;
    addBigint(a: bigint, b: bigint): bigint;
    complementBigint(a: bigint): bigint;
    subtractBigint(a: bigint, b: bigint): bigint;
}
