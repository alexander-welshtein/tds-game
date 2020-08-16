export class Interpolation {
    static increaseByDelta(value: number, limit: number, delta: number): number {
        return value > limit - delta ? limit : value + delta
    }

    static decreaseByDelta(value: number, limit: number, delta: number): number {
        return value < limit + delta ? limit : value - delta
    }
}