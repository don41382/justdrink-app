import type {PageLoad} from './$types';
import {loadAppIcon, loadImage} from "../../app";

export type GenderImages = { male: string, female: string, other: string }

/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async (): Promise<{
    iconPath: string,
    welcomePath: string,
    genderImages: GenderImages
}> => {
    return {
        iconPath: await loadAppIcon(),
        welcomePath: await loadImage("welcome/dn-water-glass.png"),
        genderImages: {
            male: await loadImage("welcome/gender/male.png"),
            female: await loadImage("welcome/gender/female.png"),
            other: await loadImage("welcome/gender/other.png"),
        },
    };
};