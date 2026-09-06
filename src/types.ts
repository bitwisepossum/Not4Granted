export type GrantStatus =
    | "Planning"
    | "Submitted"
    | "Accepted"
    | "Rejected";

export type ManuscriptStatus =
    | "Idea"
    | "Drafting"
    | "Submitted"
    | "Revision"
    | "Accepted"
    | "Published"
    | "Rejected";

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