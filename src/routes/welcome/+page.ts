import type {PageLoad} from './$types';
import {loadAppIcon, loadImage} from "../../app";

export type GenderImages = { male: string, female: string, other: string }

export type SipImages = { full: string, half: string, sip3: string, sip2: string, sip1: string }

/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async (): Promise<{
    iconPath: string,
    welcomePath: string,
    genderImages: GenderImages,
    sipImages: SipImages,
}> => {
    return {
        iconPath: await loadAppIcon(),
        welcomePath: await loadImage("welcome/dn-water-glass.png"),
        genderImages: {
            male: await loadImage("welcome/gender/male.png"),
            female: await loadImage("welcome/gender/female.png"),
            other: await loadImage("welcome/gender/other.png"),
        },
        sipImages: {
            full: await loadImage("welcome/cups/full.png"),
            half: await loadImage("welcome/cups/half.png"),
            sip3: await loadImage("welcome/cups/sip3.png"),
            sip2: await loadImage("welcome/cups/sip2.png"),
            sip1: await loadImage("welcome/cups/sip1.png"),
        }
    };
};