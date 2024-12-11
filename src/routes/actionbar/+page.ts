import type {PageLoad} from './$types';
import {loadAppIcon} from "../../app";
import {commands} from "../../bindings";

/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async () => {
    return {
        iconPath: await loadAppIcon(),
        timerStatus: await commands.getCurrentTimerStatus(),
    };
};