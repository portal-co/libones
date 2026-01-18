export class Bitwidth{
    readonly #bits: number;
   constructor(bits: number){
       this.#bits = bits;
   }
   get bits(): number{
         return this.#bits;
    }
    add(a: number, b: number): number{
        const mod = 1 << this.#bits;
        const sum = a + b;
        return (sum & (mod - 1)) + (sum >= mod ? 1 : 0);
    }
    complement(a: number): number{
        const mod = 1 << this.#bits;
        return (mod - 1) - a;
    }
    subtract(a: number, b: number): number{
        return this.add(a, this.complement(b));
    }
    addBigint(a: bigint, b: bigint): bigint{
        const mod = BigInt(1) << BigInt(this.#bits);
        const sum = a + b;
        return (sum & (mod - BigInt(1))) + (sum >= mod ? BigInt(1) : BigInt(0));
    }
    complementBigint(a: bigint): bigint{
        const mod = BigInt(1) << BigInt(this.#bits);
        return (mod - BigInt(1)) - a;
    }
    subtractBigint(a: bigint, b: bigint): bigint{
        return this.addBigint(a, this.complementBigint(b));
    }
    static{
        Object.freeze(Bitwidth.prototype);
        Object.freeze(Bitwidth); 
    }
}