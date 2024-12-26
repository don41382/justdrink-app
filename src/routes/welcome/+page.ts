import type {PageLoad} from './$types';
import {loadAppIcon, loadImage} from "../../app";

export type PainTypeImages = { upperBackImage: string, lowerBackImage: string, wristImage: string, stress: string }

/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async (): Promise<{
    iconPath: string,
    welcomePath: string,
    painTypeImages: PainTypeImages
}> => {
    return {
        iconPath: await loadAppIcon(),
        welcomePath: await loadImage("welcome/mm-welcome.png"),
        painTypeImages: {
            upperBackImage: await loadImage("welcome/upper.png"),
            lowerBackImage: await loadImage("welcome/lower.png"),
            wristImage: await loadImage("welcome/wrist.png"),
            stress: await loadImage("welcome/stress.png")
        },
    };
};