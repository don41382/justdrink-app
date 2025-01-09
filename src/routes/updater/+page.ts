import {check, type Update} from "@tauri-apps/plugin-updater";
import {loadAppIcon} from "../../app";
import type {UpdateState} from "./updateState";

function getPrereleaseHeader(): boolean {
    const prereleaseParam = new URLSearchParams(window.location.search).get("prerelease");
    return prereleaseParam !== null;
}

/** @type {import('./$types').PageLoad} */
export async function load({params}): Promise<{ iconPath: string; updateState: UpdateState }> {
    const iconPath = await loadAppIcon();
    const updateState = await check({
        headers: { "prerelease": String(getPrereleaseHeader()) }
    }).then((update: Update | null): UpdateState => {
        if (update === null) {
            return {state: "upToDate"};
        } else {
            return {state: "updateAvailable", update: update};
        }
    }).catch((e) => {
        throw Error(`Error while updating: ${e}`);
    })

    return {iconPath, updateState}
}