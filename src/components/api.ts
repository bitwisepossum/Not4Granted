import { invoke } from "@tauri-apps/api/core";
import type { Grant, NewGrant } from "../types";

export async function addGrant(grant: NewGrant): Promise<Grant> {
    return await invoke<Grant>("add_grant", { grant });
}