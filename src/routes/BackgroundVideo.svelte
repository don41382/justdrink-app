<script lang="ts">
    import {onMount} from "svelte";
    import {info, warn} from "@tauri-apps/plugin-log";

    export let videoSrc: string;
    export let backgroundVideoLoaded = false;

    let backgroundVideo: HTMLVideoElement;

    onMount(async () => {
        // Create a new source element
        const source = document.createElement('source');
        source.src = videoSrc;
        source.type = 'video/mp4';
        source.addEventListener("error", (e) => {
            info(`can't play background video: ${e}`);
            backgroundVideo.style.backgroundColor = 'white';
            backgroundVideoLoaded = true;
        })

        // Append the source to the video element
        backgroundVideo.appendChild(source);

        try {
            // Load the video
            backgroundVideo.load();
        } catch (e) {
            await info(`video error: ${e}`)
        }
    });

    function setBackgroundVideoReady() {
        if (backgroundVideo.readyState === 4) {
            backgroundVideoLoaded = true;
        }
    }
</script>

<video
        autoplay
        bind:this={backgroundVideo}
        class="absolute w-full h-full object-cover"
        loop
        muted
        on:canplay={setBackgroundVideoReady}
        on:playing={setBackgroundVideoReady}
        playsinline
        preload="auto">
    Your browser does not support the video tag.
</video>

