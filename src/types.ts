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
export type sortDirection = "asc" | "desc";

export interface GrantQuery {
    statuses?: GrantStatus[];
    funders?: string[];
    search?: string;
    sortBy?: GrantSort;
    sortDirection?: sortDirection;
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