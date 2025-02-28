import type {PageLoad} from './$types';
import {loadAppIcon, loadImage} from "../../app";

import {
    type LicenseData,
    type WelcomeWizardMode,
    commands, type WelcomeLoadSettings,
} from "../../bindings";

import {info} from "@tauri-apps/plugin-log";

export type GenderImages = { male: string, female: string, other: string }
export type SipImages = { full: string, half: string, sip3: string, sip2: string, sip1: string }
export type ReminderImages = { woman: string, man: string }

export interface WelcomeImages {
    welcomePath: string,
    gender: GenderImages,
    sip: SipImages,
    reminder: ReminderImages

}

function getMode(): WelcomeWizardMode {
    let mode = new URLSearchParams(window.location.search).get("mode");
    if (mode === "Complete" || mode === "OnlySipSettings" || mode === "OnlyPayment") {
        return mode;
    } else {
        return "Complete";
    }
}


/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async (): Promise<{
    iconPath: string,
    deviceId: string,
    settings: WelcomeLoadSettings,
    licenseData: Promise<LicenseData>,
    welcomeMode: WelcomeWizardMode,
    images: WelcomeImages,
}> => {
    await info("welcome load")
    const deviceId = await commands.getDeviceId()
    return {
        iconPath: await loadAppIcon(),
        deviceId: deviceId,
        settings: await commands.welcomeLoadSettings(),
        licenseData: commands.requestLicenseStatus(),
        welcomeMode: getMode(),
        images: {
            welcomePath: await loadImage("welcome/dn-water-glass.png"),
            gender: {
                male: await loadImage("welcome/gender/male.png"),
                female: await loadImage("welcome/gender/female.png"),
                other: await loadImage("welcome/gender/other.png"),
            },
            sip: {
                full: await loadImage("welcome/cups/full.png"),
                half: await loadImage("welcome/cups/half.png"),
                sip3: await loadImage("welcome/cups/sip3.png"),
                sip2: await loadImage("welcome/cups/sip2.png"),
                sip1: await loadImage("welcome/cups/sip1.png"),
            },
            reminder: {
                man: await loadImage("welcome/reminder/man.png"),
                woman: await loadImage("welcome/reminder/woman.png"),
            }
        }
    }
        ;
};