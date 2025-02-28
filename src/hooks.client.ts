import {warn} from "@tauri-apps/plugin-log";
import {commands} from "./bindings";
import {getCurrentWindow} from "@tauri-apps/api/window";
import type {HandleClientError} from "@sveltejs/kit";

export const handleError: HandleClientError = async ({ error, event, message }) => {
    await warn(`Client-side error: ${message}, with event: ${event.route}, ${event.url}, error: ${error}`);
    await commands.alertLogClientError(
        "Client error",
        `There was an error while running UI. If the error persists, please contact me at info@rocket-solutions.de. Error: ${message}`,
        `${error}`
    );
    await getCurrentWindow().close()
}