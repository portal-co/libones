export class Bitwidth {
    #bits;
    constructor(bits) {
        this.#bits = bits;
    }
    get bits() {
        return this.#bits;
    }
    add(a, b) {
        const mod = 1 << this.#bits;
        const sum = a + b;
        return (sum & (mod - 1)) + (sum >= mod ? 1 : 0);
    }
    complement(a) {
        const mod = 1 << this.#bits;
        return (mod - 1) - a;
    }
    subtract(a, b) {
        return this.add(a, this.complement(b));
    }
    addBigint(a, b) {
        const mod = BigInt(1) << BigInt(this.#bits);
        const sum = a + b;
        return (sum & (mod - BigInt(1))) + (sum >= mod ? BigInt(1) : BigInt(0));
    }
    complementBigint(a) {
        const mod = BigInt(1) << BigInt(this.#bits);
        return (mod - BigInt(1)) - a;
    }
    subtractBigint(a, b) {
        return this.addBigint(a, this.complementBigint(b));
    }
    static {
        Object.freeze(Bitwidth.prototype);
        Object.freeze(Bitwidth);
    }
}
