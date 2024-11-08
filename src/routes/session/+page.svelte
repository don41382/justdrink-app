<script lang="ts">
    import {error, info} from '@tauri-apps/plugin-log';
    import {
        commands,
        type SessionEndingReason,
        type SessionDetail,
        type Exercise,
        type LicenseInfo
    } from '../../bindings';
    import {getCurrentWindow} from '@tauri-apps/api/window';
    import {onDestroy, onMount} from 'svelte';
    import {fade} from 'svelte/transition';
    import Icon from '@iconify/svelte';
    import AdviceMessage from "./AdviceMessage.svelte";
    import VideoPlayer from "./VideoPlayer.svelte";
    import * as tauri_path from '@tauri-apps/api/path';
    import {convertFileSrc} from "@tauri-apps/api/core";
    import AudioPlayer from "./AudioPlayer.svelte";
    import BackgroundVideo from "../BackgroundVideo.svelte";

    info("Initialized Session Window")

    let {data} = $props()

    const fadeOutDurationSeconds = 2

    const exercise: Exercise = data.sessionDetail.exercise
    let license: LicenseInfo = data.sessionDetail.license_info

    let countdownSeconds: number | undefined = $state(data.sessionDetail.exercise.duration_s)
    let countdownInterval: number | undefined = $state(undefined);
    let backgroundLoadingFinished: boolean = $state(false);

    let music: AudioPlayer;
    let finishSound: HTMLAudioElement;

    let show = $state(true);

    onMount(async () => {
        await info(`mount session window: ${data.backgroundVideoSrc}`)
        await init().catch((err) => {
            commands.alertLogClientError(
                "Session Error",
                `Session could not be started. Reason: ${err}`,
                `error during init: ${err}`);
        })
    });

    async function init() {
        finishSound.src = data.finishSound
        music.play();
        startTimer();

        const window = getCurrentWindow()
        await window.setFocus()
    }

    function startTimer() {
        countdownInterval = window.setInterval(() => {
            if (countdownSeconds) {
                countdownSeconds -= 1;
                if (countdownSeconds <= 0) {
                    countdownSeconds = 0
                    clearInterval(countdownInterval);
                }

                if (countdownSeconds < 2) {
                    finishSound.play();
                }

                /*if (countdownSeconds < 1) {
                    show = false;
                }*/
            }
        }, 1000);
    }

    function cleanup() {
        if (countdownInterval) {
            clearInterval(countdownInterval);
        }
    }

    onDestroy(() => {
        cleanup();
    });

    function beforeCloseApp() {
        music.fadeOut(fadeOutDurationSeconds * 1000);
    }

    function closeApp(reason: SessionEndingReason) {
        cleanup();
        commands.endSession(reason);
    }

    function onKeyDown(e: KeyboardEvent) {
        switch (e.key) {
            case "Escape":
                closeApp("UserEscape");
                break;
        }
    }


    function formatCountdown(seconds: number): string {
        const minutes = Math.floor(seconds / 60);
        const secs = seconds % 60;
        return `${String(minutes).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
    }

</script>

<svelte:window on:keydown|preventDefault={onKeyDown}/>

{#if show}
    <div aria-pressed="true"
         class="{backgroundLoadingFinished ? 'video-background-ready' : 'video-not-ready'} bg-transparent h-screen flex flex-col justify-between items-center overflow-hidden cursor-default backdrop-blur-lg"
         out:fade={{duration: fadeOutDurationSeconds * 1000}} onoutrostart={beforeCloseApp}
         onoutroend={() => closeApp("EndOfTime")}>

        <AudioPlayer bind:this={music} filename="session-01.mp3" initialVolume={0.4}/>
        <audio bind:this={finishSound} src="" preload="auto"></audio>

        <BackgroundVideo videoSrc={data.backgroundVideoSrc} bind:backgroundVideoLoaded={backgroundLoadingFinished}/>

        <div class="absolute top-9 left-10">
            {#if exercise !== undefined && countdownSeconds !== undefined}
                <h1 class="text-6xl text-[#cc006e] tracking-tight font-bold mb-4">{exercise.title}</h1>
                <h1 class="text-4xl text-[#b3b0fd] font-light tracking-wide w-1/2 mb-4">{exercise.description}</h1>
                {#if license && (license.status === 'Paid' || license.status === 'Invalid')}
                    <span class="text-white/50 font-light">{license.message}</span>
                {/if}
            {/if}
        </div>
        <div class="absolute bottom-0 z-10 w-full flex items-center justify-center">
            {#if exercise}
                <div out:fade={{ duration: 1000 }}>
                    <VideoPlayer filename={data.sessionDetail.exercise.id} class="max-h-[80vh] h-auto"/>
                </div>
            {/if}
        </div>
        <div class="absolute top-9 right-10 z-20 text-gray-300 flex flex-col items-center">
            <div class="text-5xl font-light mb-6">
                {#if countdownSeconds !== undefined}
                <span in:fade={{ duration: 1000 }}>
                    {formatCountdown(countdownSeconds)}
                </span>
                {/if}
            </div>

        </div>
        <div class="absolute bottom-7 left-10 z-20 w-1/4 text-white/40 flex flex-col">
            {#if exercise !== undefined && countdownSeconds !== undefined}
                <div class="text-xl font-light">
                    <AdviceMessage advices={exercise.advices}/>
                </div>
            {/if}
        </div>
        <div class="absolute bottom-7 right-10 z-20 flex flex-col items-center">
            <button class="bg-white bg-opacity-5 hover:bg-white hover:bg-opacity-20 font-bold py-2 px-4 rounded-l border text-gray-300 border-gray-200 inline-flex items-center"
                    onclick={() => closeApp("UserEscape")}>
                Skip Motion
                <Icon icon="mdi-light:arrow-right" class="ml-2 size-7"/>
            </button>
        </div>

    </div>
{/if}

<style>
    .video-background-ready {
        opacity: 1.0;
        transition: opacity 1s;
    }

    .video-not-ready {
        opacity: 0;
    }
</style>