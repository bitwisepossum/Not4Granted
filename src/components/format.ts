import type { Locale } from "../types";

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

//
// Formats a date string into a human-readable format.
// If the input value is undefined or empty, it returns an em dash (—).
//
export function formatDate(date?: string, locale: Locale = "en-US"): string {
    if (!date) return "\u2014"; // em dash

    return new Intl.DateTimeFormat(locale, {
        year: "numeric",
        month: "long",
        day: "numeric",
    }).format(new Date(date));
}