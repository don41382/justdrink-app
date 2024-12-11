import type {PageLoad} from './$types';
import {loadAppIcon, loadImage} from "../../app";

/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async () => {
    return {
        iconPath: await loadAppIcon(),
        welcomePath: await loadImage("mm-welcome.png")
    };
};