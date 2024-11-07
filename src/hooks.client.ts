import {warn} from "@tauri-apps/plugin-log";
import {commands} from "./bindings";


export async function handleError({error}) {
    await warn(`Client-side error: ${error}`);
    await commands.alertLogClientError(
        "Client error",
        "There was an error while running UI. If the error persists, please contact me at info@rocket-solutions.de. Thanks.",
        `${error}`
    );
}