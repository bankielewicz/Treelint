/**
 * Test fixture: Simple TypeScript functions for symbol extraction testing.
 */

export function greet(name: string): string {
    return `Hello, ${name}!`;
}

function add(a: number, b: number): number {
    return a + b;
}

const multiply = (x: number, y: number): number => {
    return x * y;
};

export async function fetchData(url: string): Promise<object> {
    return { url, data: null };
}

// Generic function
function identity<T>(arg: T): T {
    return arg;
}
