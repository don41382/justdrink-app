import {check, type Update} from "@tauri-apps/plugin-updater";
import type {StateType} from "./stateType";
import {loadAppIcon} from "../../app";
import type {UpdateAvailable, UpdateState} from "./updateState";
import {commands} from "../../bindings";


/** @type {import('./$types').PageLoad} */
export async function load({params}): Promise<{ iconPath: string; updateState: UpdateState }> {
    const iconPath = await loadAppIcon();
    const updateState = await check().then((update: Update | null): UpdateState => {
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