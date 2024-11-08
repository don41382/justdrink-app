import type {PageLoad} from './$types';
import {loadAppIcon} from "../../app";
import * as tauri_path from "@tauri-apps/api/path";
import {convertFileSrc} from "@tauri-apps/api/core";
import {commands} from "../../bindings";

/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async () => {
    const resource_dir = await tauri_path.resourceDir();
    const sessionDetail = await commands.loadSessionDetails();
    if (sessionDetail === null || sessionDetail === undefined) {
        throw new Error("No Session found. Please try again later.")
    }
    return {
        sessionDetail,
        iconPath: await loadAppIcon(),
        sessionVideoSrc: convertFileSrc(`${resource_dir}/videos/${sessionDetail.exercise.id}.mov`),
        backgroundVideoSrc: convertFileSrc(`${resource_dir}/videos/bg-h264.mov`),
        background: convertFileSrc(`${resource_dir}/audio/session-01.mp3`),
        finishSound: convertFileSrc(`${resource_dir}/audio/session-end.mp3`)
    };
};