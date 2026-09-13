export const grantStatuses = [
    "Planning",
    "Submitted",
    "Accepted",
    "Rejected"
] as const;

export type GrantStatus =
    typeof grantStatuses[number];

export const manuscriptStatuses = [
    "Idea",
    "Drafting",
    "Submitted",
    "Revision",
    "Accepted",
    "Published",
    "Rejected"
] as const;

export type ManuscriptStatus =
    typeof manuscriptStatuses[number];

export type View =
    | "dashboard"
    | "grants"
    | "manuscripts"
    | "add-grant"
    | "add-manuscript"
    | "edit-grant"
    | "edit-manuscript";

export interface Grant {
    id: number;

    name: string;
    funder: string;
    callName?: string;

    status: GrantStatus;

    amountRequested?: number;
    amountReceived?: number;
    currency: string;

    deadline?: string;
    submittedAt?: string;
    decisionAt?: string;

    notes?: string;
}

export type NewGrant = Omit<Grant, "id">;

export interface Manuscript {
    id: number;

    title: string;
    shortName?: string;
    journal?: string;

    status: ManuscriptStatus;
    nextAction?: string;

    submittedAt?: string;
    decisionAt?: string;
    publishedAt?: string;

    doi?: string;
    notes?: string;
}

export type NewManuscript = Omit<Manuscript, "id">;

export type GrantSort = "deadline" | "name" | "status" | "funder";
export type SortDirection = "asc" | "desc";

export interface GrantQuery {
    id?: number;
    statuses?: GrantStatus[];
    funders?: string[];
    search?: string;
    sortBy?: GrantSort;
    sortDirection?: SortDirection;
}
export type ManuscriptSort =
    | "title"
    | "status"
    | "journal";

export interface ManuscriptQuery {
    id?: number;
    statuses?: ManuscriptStatus[];
    journals?: string[];
    search?: string;
    sortBy?: ManuscriptSort;
    sortDirection?: SortDirection;
}

export function defaultManuscriptQuery(): ManuscriptQuery {
    return {
        statuses: [],
        journals: [],
        search: "",
        sortBy: "title",
        sortDirection: "asc"
    };
}

export function defaultGrantQuery(): GrantQuery {
    return {
        statuses: [],
        funders: [],
        search: "",
        sortBy: "deadline",
        sortDirection: "asc"
    };
}

export const LocaleNames = {
    "en-US": "English (United States)",
    "fi-FI": "Finnish (Finland)",
    "sv-SE": "Swedish (Sweden)",
    "es-ES": "Spanish (Spain)",
    "fr-FR": "French (France)",
    "de-DE": "German (Germany)",
    "it-IT": "Italian (Italy)",
    "pt-PT": "Portuguese (Portugal)",
    "nl-NL": "Dutch (Netherlands)",
    "pl-PL": "Polish (Poland)",
    "ru-RU": "Russian (Russia)",
    "ja-JP": "Japanese (Japan)",
    "zh-CN": "Chinese (China)",
    "ko-KR": "Korean (South Korea)",
    "ar-SA": "Arabic (Saudi Arabia)",
    "he-IL": "Hebrew (Israel)",
    "tr-TR": "Turkish (Türkiye)",
    "cs-CZ": "Czech (Czechia)",
    "da-DK": "Danish (Denmark)",
    "el-GR": "Greek (Greece)",
    "hu-HU": "Hungarian (Hungary)",
    "id-ID": "Indonesian (Indonesia)",
    "ms-MY": "Malay (Malaysia)",
    "no-NO": "Norwegian (Norway)",
    "ro-RO": "Romanian (Romania)",
    "sk-SK": "Slovak (Slovakia)",
    "th-TH": "Thai (Thailand)",
    "uk-UA": "Ukrainian (Ukraine)",
    "vi-VN": "Vietnamese (Vietnam)"
};

export type Locale = keyof typeof LocaleNames;

export const ThemeName = {
    light: "Light",
    dark: "Dark",
    system: "Use system setting"
} as const;

export type Theme = keyof typeof ThemeName;

export type Settings = {
    locale: Locale;
    theme: Theme;
    version: string;
    currency: string;
};