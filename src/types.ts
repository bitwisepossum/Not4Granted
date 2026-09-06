export type GrantStatus = "Planning" | "Submitted" | "Accepted" | "Rejected";
export type ManuscriptStatus = "Idea" | "Drafting" | "Submitted" | "Revision" | "Accepted";
export type View = "dashboard" | "grants" | "manuscripts" | "add-grant" | "add-manuscript";

export interface Grant {
    id: number;
    name: string;
    funder: string;
    deadline: string;
    amount: string;
    status: GrantStatus;
};

export type NewGrant = Omit<Grant, "id">;

export interface Manuscript {
    id: number;
    title: string;
    journal: string;
    status: ManuscriptStatus;
    nextAction: string;
};

export type NewManuscript = Omit<Manuscript, "id">;