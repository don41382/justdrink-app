import {getCurrentWindow, PhysicalSize} from "@tauri-apps/api/window";
import {debug, warn} from "@tauri-apps/plugin-log";

export async function fitAndShowWindow(contentDiv: HTMLElement) {
    await debug("fitAndShowWindow")
    let rect = contentDiv.getBoundingClientRect()
    const factor = window.devicePixelRatio;
    const width: number = Math.ceil(rect.width * factor);
    const height: number = Math.ceil(rect.height * factor);
    let size = new PhysicalSize(width, height);
    await getCurrentWindow().setSize(size).catch(async (e) => {
        await getCurrentWindow().show();
        await warn(`failed to set window size: ${e}, will try to set window size to: ${width}x${height}`)
    }).then(async () => {
        await getCurrentWindow().center();
        await getCurrentWindow().show();
        await getCurrentWindow().setFocus();
    })
}
