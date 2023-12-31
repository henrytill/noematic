export const SCHEMA_VERSION = '0.1.0';

/**
 * Abbreviates a string to a given length if it is longer than the length.
 * @param {string} str
 * @param {number} length
 * @returns {string}
 */
export function abbreviate(str, length) {
    if (str.length <= length) return str;
    return str.slice(0, length - 3) + '...';
}
