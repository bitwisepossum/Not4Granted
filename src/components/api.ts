import { invoke } from "@tauri-apps/api/core";
import type { Grant, NewGrant, Manuscript, NewManuscript } from "../types";

/*
    * API functions for Grants.
*/
export async function addGrant(grant: NewGrant): Promise<Grant> {
    return await invoke<Grant>("add_grant", { grant });
}

export async function getGrants(): Promise<Grant[]> {
    return await invoke<Grant[]>("get_grants");
}

export async function getGrantById(id: number): Promise<Grant | undefined> {
    return await invoke<Grant | undefined>("get_grant_by_id", { grantId: id });
}

export async function updateGrant(grant: Grant): Promise<void> {
    return await invoke<void>("update_grant", { grant });
}

export async function deleteGrant(id: number): Promise<void> {
    return await invoke<void>("delete_grant", { grantId: id });
}

export async function getAcceptedGrants(): Promise<Grant[]> {
    return await invoke<Grant[]>("get_accepted_grants");
}

/*
    * API functions for Manuscripts.
*/
export async function addManuscript(manuscript: NewManuscript): Promise<Manuscript> {
    return await invoke<Manuscript>("add_manuscript", { manuscript });
}

export async function getManuscripts(): Promise<Manuscript[]> {
    return await invoke<Manuscript[]>("get_manuscripts");
}

export async function getManuscriptById(id: number): Promise<Manuscript | undefined> {
    return await invoke<Manuscript | undefined>("get_manuscript_by_id", { manuscriptId: id });
}

export async function updateManuscript(manuscript: Manuscript): Promise<void> {
    return await invoke<void>("update_manuscript", { manuscript });
}

export async function deleteManuscript(id: number): Promise<void> {
    return await invoke<void>("delete_manuscript", { manuscriptId: id });
}