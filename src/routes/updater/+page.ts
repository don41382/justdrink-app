import * as tauri_path from "@tauri-apps/api/path";
import {convertFileSrc} from "@tauri-apps/api/core";
import {check, type Update} from "@tauri-apps/plugin-updater";
import type {StateType} from "./stateType";
import {loadAppIcon} from "../../app";

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
    const iconPath = await loadAppIcon();
    const { update, error, initialState } = await check().then((update) => {
        const error: String | null = null;
        const initialState: StateType = update == null ? "newest" : "init"
        return { update, error, initialState }
    }).catch((e) => {
        const error: string = `${e}`
        const update: Update | null = null;
        const initialState: StateType = "error"
        return { update, error, initialState };
    })

    return {
        iconPath,
        update,
        initialState,
        error
    }
}