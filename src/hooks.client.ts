import {warn} from "@tauri-apps/plugin-log";
import {commands} from "./bindings";


export async function handleError({error}) {
    await warn(`Client-side error: ${error}`);
    await commands.alertLogClientError(`${error}`);
}