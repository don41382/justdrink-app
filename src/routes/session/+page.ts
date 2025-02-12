import type {PageLoad} from './$types';
import {loadAppIcon} from "../../app";
import * as tauri_path from "@tauri-apps/api/path";
import {convertFileSrc} from "@tauri-apps/api/core";
import type {DrinkCharacter, SipSize} from "../../bindings";
import {DrinkCharacters} from "../DrinkCharacters";
import type {Size} from "@tauri-apps/api/dpi";

export interface DrinkAudio {
    personas: Record<DrinkCharacter, AudioPersona>,
    sparkling: string[]
}

export interface GlassVideo {
    mov: string,
    webm: string,
}

export interface AudioPersona {
    sips: string[],
    ahh: string[]
}

function toCharacter(tab: string | null): DrinkCharacter {
    if (tab != null && tab == "YoungMan" || tab == "YoungWoman") {
        return tab
    } else {
        return "YoungWoman"
    }
}

function toSipSize(param: string | null): SipSize {
    if (param != null) {
        return param as SipSize
    } else {
        return "BigSip"
    }
}

function createAudioPaths(baseName: string, count: number, resourceDir: string): string[] {
    return Array.from({length: count}, (_, i) => convertFileSrc(`${resourceDir}/audio/drink/${baseName}-0${i + 1}.mp3`))
}

function toAudioPersonaData(character: DrinkCharacter, resourceDir: string): AudioPersona {
    switch (character) {
        case "YoungMan":
            return {
                sips: createAudioPaths('man-gulp', 6, resourceDir),
                ahh: createAudioPaths('man-ahh', 1, resourceDir),
            }
        case "YoungWoman":
            return {
                sips: createAudioPaths('woman-gulp', 8, resourceDir),
                ahh: createAudioPaths('woman-ahh', 3, resourceDir),
            }
    }
}

function linkAudio(resourceDir: string): DrinkAudio {
    const personas = DrinkCharacters.all.reduce((acc, character) => {
        acc[character] = toAudioPersonaData(character, resourceDir);
        return acc;
    }, {} as Record<string, AudioPersona>);
    return {
        personas: personas,
        sparkling: createAudioPaths('sparkling', 1, resourceDir)
    }
}


export const load: PageLoad = async () => {
    const resourceDir = await tauri_path.resourceDir();
    const params = new URLSearchParams(window.location.search);
    let drinkCharacter: DrinkCharacter = toCharacter(params.get("character"));
    let sipSize: SipSize = toSipSize(params.get("sip_size"))

    return {
        iconPath: await loadAppIcon(),
        drinkAudio: linkAudio(resourceDir),
        video: {
            mov: convertFileSrc(`${resourceDir}/videos/full.mov`),
            webm: convertFileSrc(`${resourceDir}/videos/full.webm`),
        },
        selectedDrinkCharacter: drinkCharacter,
        sipSize: sipSize
    };
};