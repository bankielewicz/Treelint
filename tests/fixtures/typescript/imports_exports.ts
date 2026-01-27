/**
 * Test fixture: TypeScript imports and exports for symbol extraction testing.
 */

import { readFile } from "fs/promises";
import path from "path";
import type { Buffer } from "buffer";

// Constants
export const MAX_RETRIES = 3;
export const DEFAULT_TIMEOUT = 30000;
const INTERNAL_SECRET = "hidden";

// Variables
let counter = 0;
var legacyVar = "old style";

// Type exports
export type UserId = string;
export interface User {
    id: UserId;
    name: string;
}

// Function exports
export function createUser(name: string): User {
    return { id: String(++counter), name };
}

// Default export
export default class ApiClient {
    private baseUrl: string;

    constructor(baseUrl: string) {
        this.baseUrl = baseUrl;
    }
}

// Re-exports
export { readFile as read } from "fs/promises";
