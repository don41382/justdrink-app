<script lang="ts">
    import {onMount} from "svelte";

    export let filename: string;
    let className = '';
    export {className as class};
    import * as tauri_path from '@tauri-apps/api/path';
    import {convertFileSrc} from "@tauri-apps/api/core";

    let videoElement: HTMLVideoElement;
    let videoReady = false;

    function videoSource(src: string, type: string) {
        let sourceElement = document.createElement('source');
        sourceElement.src = src;
        sourceElement.type = type;
        return sourceElement;
    }

    onMount(async () => {
        let resource_dir = await tauri_path.resourceDir();
        videoElement.appendChild(
            videoSource(convertFileSrc(`${resource_dir}/videos/${filename}.mov`), 'video/mp4; codecs="hvc1"')
        )
        videoElement.appendChild(
            videoSource(convertFileSrc(`${resource_dir}/videos/${filename}.webm`), 'video/webm')
        )
    })

    function setVideoReady() {
        videoReady = true;
    }

    $: ready = videoReady;
</script>

<style>
    .video-ready {
        opacity: 1;
        transition: opacity 4s;
    }

    .video-not-ready {
        opacity: 0;
    }
</style>

<video
        autoplay
        bind:this={videoElement}
        class="{className} {ready ? 'video-ready' : 'video-not-ready'}"
        loop muted on:canplay={setVideoReady} playsinline preload="metadata">
</video>