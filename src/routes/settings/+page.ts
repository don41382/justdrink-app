import {commands} from '../../bindings';
import type {PageLoad} from './$types';
import {debug} from "@tauri-apps/plugin-log";

/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async () => {
    return {
        settings: await commands.loadSettings()
    };
};