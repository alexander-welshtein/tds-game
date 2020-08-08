export class MathSet {
    static randomNumber() {
        return Math.floor(Math.random() * Math.floor(Number.MAX_VALUE))
    }

    static randomNumberByMax(max: number) {
        return Math.floor(Math.random() * Math.floor(max))
    }

    static lerp(a: number, b: number, t: number) {
        t = t < 0 ? 0 : t
        t = t > 1 ? 1 : t
        return a + (b - a) * t
    }
}