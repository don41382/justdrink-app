<script lang="ts">
    import {onMount} from "svelte";
    import type {GlassVideo} from "./+page";

    let { video } : { video: GlassVideo } = $props()
    const startTimeSeconds = 0.5

    let videoElement: HTMLVideoElement;

    function videoSource(src: string, type: string) {
        let sourceElement = document.createElement('source');
        sourceElement.src = src;
        sourceElement.type = type;
        return sourceElement;
    }

    export async function play() {
        videoElement.currentTime = startTimeSeconds
        await videoElement.play()
    }

    onMount(async () => {
        videoElement.appendChild(
            videoSource(video.mov, 'video/mp4; codecs="hvc1"')
        )
        videoElement.appendChild(
            videoSource(video.webm, 'video/webm')
        )
        videoElement.currentTime = startTimeSeconds
    })

</script>

<video
        bind:this={videoElement}
        class="size-72"
        muted playsinline preload="metadata">
</video>