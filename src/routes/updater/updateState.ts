import type {Update} from "@tauri-apps/plugin-updater";

export interface UpToDate {
    state: "upToDate"
}

export interface UpdateAvailable {
    state: "updateAvailable",
    update: Update
}

export type UpdateState = UpToDate | UpdateAvailable