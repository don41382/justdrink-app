import * as tauri_path from "@tauri-apps/api/path";
import {convertFileSrc} from "@tauri-apps/api/core";

export async function loadAppIcon() {
    const resourceDir = await tauri_path.resourceDir();
    return convertFileSrc(`${resourceDir}/icons/128x128.png`);
}

export async function loadImage(img: string) {
    const resourceDir = await tauri_path.resourceDir();
    return convertFileSrc(`${resourceDir}/icons/${img}`);
}