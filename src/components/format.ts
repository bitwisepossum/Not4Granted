// 
// Converts a currency value between float and integer representations.
// For example, 10.50 euros can be represented as 1050 cents (integer).
//
export function convertCurrency(
    value: number,
    toCents = false
): number {
    if (toCents) {
        return Math.round(value * 100);
    }

    return value / 100;
}