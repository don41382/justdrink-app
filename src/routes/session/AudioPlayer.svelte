<script lang="ts">
    import {onMount} from "svelte";
    import * as tauri_path from "@tauri-apps/api/path";
    import {convertFileSrc} from "@tauri-apps/api/core";
    import {info} from "@tauri-apps/plugin-log";

    export let filename = "";            // MP3 file to play
    export let initialVolume = 1.0;  // Initial volume (1.0 is max)

    let audio: HTMLAudioElement;

    onMount(async () => {
        let resource_dir = await tauri_path.resourceDir();
        audio.src = convertFileSrc(`${resource_dir}/audio/${filename}`)
        await info(`set source ${audio.src}`)
    })

    // Function to play the audio
    export function play() {
        if (!audio) return;
        audio.volume = initialVolume;
        info("play")
        audio.play();
    }

    // Function to pause the audio
    function pause() {
        if (!audio) return;
        audio.pause();
    }

    // Function to fade out the audio externally
    export function fadeOut(duration) {
        if (!audio) return;
        let fadeInterval = 50; // Frequency of volume updates
        let fadeStep = audio.volume / (duration / fadeInterval); // How much to decrease per step

        let fadeAudio = setInterval(() => {
            if (audio.volume > 0.01) {
                audio.volume = Math.max(audio.volume - fadeStep, 0); // Reduce volume
            } else {
                clearInterval(fadeAudio);
                audio.pause();
            }
        }, fadeInterval);
    }
</script>

<audio bind:this={audio} src=""></audio>