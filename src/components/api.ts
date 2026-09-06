import { invoke } from "@tauri-apps/api/core";
import type { Grant, NewGrant } from "../types";

export async function addGrant(grant: NewGrant): Promise<Grant> {
    return await invoke<Grant>("add_grant", { grant });
}

export async function getGrants(): Promise<Grant[]> {
    return await invoke<Grant[]>("get_grants");
}

export async function getManuscripts(): Promise<any[]> {
    return await invoke<any[]>("get_manuscripts");
}