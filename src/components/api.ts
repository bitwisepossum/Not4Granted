import { invoke } from "@tauri-apps/api/core";
import type { Grant, NewGrant, Manuscript, NewManuscript, GrantQuery, Settings } from "../types";

/*
    * API functions for Grants.
*/
export async function addGrant(newGrant: NewGrant): Promise<Grant> {
    return await invoke<Grant>("add_grant", {newGrant});
}

export async function getGrants(): Promise<Grant[]> {
    return await invoke<Grant[]>("get_grants");
}

export async function updateGrant(grant: Grant): Promise<void> {
    return await invoke<void>("update_grant", { grant });
}

export async function deleteGrant(id: number): Promise<void> {
    return await invoke<void>("delete_grant", { grantId: id });
}

export function getFilteredGrants(query: GrantQuery): Promise<Grant[]> {
    return invoke<Grant[]>("get_filtered_grants", {query});
}

export function getGrantById(id: number): Promise<Grant | undefined> {
    return invoke<Grant | undefined>("get_grant_by_id", { grantId: id });
}

/*
    * API functions for Manuscripts.
*/
export async function addManuscript(newManuscript: NewManuscript): Promise<Manuscript> {
    return await invoke<Manuscript>("add_manuscript", {newManuscript});
}

export async function getManuscripts(): Promise<Manuscript[]> {
    return await invoke<Manuscript[]>("get_manuscripts");
}

export async function updateManuscript(manuscript: Manuscript): Promise<void> {
    return await invoke<void>("update_manuscript", { manuscript });
}

export async function deleteManuscript(id: number): Promise<void> {
    return await invoke<void>("delete_manuscript", { manuscriptId: id });
}

export function getFilteredManuscripts(query: GrantQuery): Promise<Manuscript[]> {
    return invoke<Manuscript[]>("get_filtered_manuscripts", {query});
}

export function getManuscriptById(id: number): Promise<Manuscript | undefined> {
    return invoke<Manuscript | undefined>("get_manuscript_by_id", { manuscriptId: id });
}

/*
    * API functions for Settings.
*/
export async function getSettings(): Promise<Settings> {
    return await invoke<Settings>("get_settings");
}

export function saveSettings(settings: Settings): Promise<Settings> {
    return invoke<Settings>("save_settings", { settings });
}