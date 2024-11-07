import type {PageLoad} from './$types';
import {loadAppIcon} from "../../app";

/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async () => {
    return {
        iconPath: await loadAppIcon()
    };
};