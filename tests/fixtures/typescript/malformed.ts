/**
 * Test fixture: Malformed TypeScript file for error handling testing.
 */

// Valid function
export function validFunction(): string {
    return "I work!";
}

// Syntax error - missing closing brace
function brokenFunction() {
    return "I don't work"


// Another valid function after the error
function anotherValidFunction(x: number): number {
    return x * 2;
}

// Incomplete class
class IncompleteClass {
    method(

// Valid class after error
class ValidClass {
    method(): boolean {
        return true;
    }
}
