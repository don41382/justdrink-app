import {warn} from "@tauri-apps/plugin-log";
import {commands} from "./bindings";
import {getCurrentWindow} from "@tauri-apps/api/window";


export async function handleError({error}) {
    await warn(`Client-side error: ${error}`);
    await commands.alertLogClientError(
        "Client error",
        "There was an error while running UI. If the error persists, please contact me at info@rocket-solutions.de. Thanks.",
        `${error}`
    );
    await getCurrentWindow().close();
}