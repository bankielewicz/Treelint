/**
 * Test fixture: TypeScript classes with methods for symbol extraction testing.
 */

export class Calculator {
    private value: number;

    constructor(initialValue: number = 0) {
        this.value = initialValue;
    }

    public add(x: number): number {
        this.value += x;
        return this.value;
    }

    public subtract(x: number): number {
        this.value -= x;
        return this.value;
    }

    protected reset(): void {
        this.value = 0;
    }
}

interface Point {
    x: number;
    y: number;
}

class Point2D implements Point {
    constructor(public x: number, public y: number) {}

    distanceFromOrigin(): number {
        return Math.sqrt(this.x ** 2 + this.y ** 2);
    }
}

abstract class Shape {
    abstract area(): number;
}
