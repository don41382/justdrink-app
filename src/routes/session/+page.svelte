<script lang="ts">
    import {info} from '@tauri-apps/plugin-log';
    import {commands, type SessionDetail} from '../../bindings';
    import {getCurrentWindow} from '@tauri-apps/api/window';
    import {onDestroy, onMount} from 'svelte';
    import {fade} from 'svelte/transition';
    import Icon from '@iconify/svelte';
    import AdviceMessage from "./AdviceMessage.svelte";
    import VideoPlayer from "./VideoPlayer.svelte";
    import * as tauri_path from '@tauri-apps/api/path';
    import {convertFileSrc} from "@tauri-apps/api/core";
    import AudioPlayer from "./AudioPlayer.svelte";

    const fadeOutDurationSeconds = 2

    let session: SessionDetail | undefined = undefined;
    let countdownSeconds: number | undefined;
    let countdownInterval: number | undefined;

    let music: AudioPlayer;
    let finishSound: HTMLAudioElement;

    let backgroundVideo: HTMLVideoElement;
    let backgroundVideoReady = false;

    let show = true;

    info("Initialized Session Window")

    function setup(details: SessionDetail) {
        session = details
        countdownSeconds = details.duration_s

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

                if (countdownSeconds < 1) {
                    show = false;
                }
            }
        }, 1000);
    }

    onMount(async () => {
        let resource_dir = await tauri_path.resourceDir();
        backgroundVideo.src = convertFileSrc(`${resource_dir}/videos/bg-h264.mov`)
        finishSound.src = convertFileSrc(`${resource_dir}/audio/session-end.mp3`)
        music.play();

        let res = await commands.loadSessionDetails();
        if (res.status === "ok") {
            setup(res.data);
        } else {
            alert(`An error occurred while loading session details: ${res.error}`);
        }

        const window = getCurrentWindow()
        await window.setFocus()
    });

    function setBackgroundVideoReady() {
        if (backgroundVideo.readyState === 4) {
            backgroundVideoReady = true
        }
    }

    function cleanup() {
        if (countdownInterval) {
            clearInterval(countdownInterval);
        }
        session = undefined;
    }


    onDestroy(() => {
        cleanup();
    });

    function beforeCloseApp() {
        music.fadeOut(fadeOutDurationSeconds * 1000);
    }

    function closeApp() {
        cleanup();
        commands.closeWindow();
    }

    function onKeyDown(e: KeyboardEvent) {
        switch (e.key) {
            case "Escape":
                closeApp();
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
         class="{backgroundVideoReady ? 'video-background-ready' : 'video-not-ready'} bg-transparent h-screen flex flex-col justify-between items-center overflow-hidden cursor-default"
         out:fade={{duration: fadeOutDurationSeconds * 1000}} on:outrostart={beforeCloseApp} on:outroend={closeApp}>

        <AudioPlayer bind:this={music} filename="session-01.mp3" initialVolume={0.4}/>
        <audio bind:this={finishSound} src="" preload="auto"></audio>

        <video
                autoplay
                bind:this={backgroundVideo}
                class="absolute w-full h-full object-cover blur-sm"
                loop muted on:canplay={setBackgroundVideoReady} playsinline
                preload="auto">
            <source src="" type="video/mp4">
            Your browser does not support the video tag.
        </video>

        <div class="relative z-20 flex flex-col">
            {#if session !== undefined && countdownSeconds !== undefined}
                <div class="flex-none text-center mt-20 px-48">
                    <h1 class="text-8xl text-mm-blue font-bold mb-4">{session.title}</h1>
                    <h1 class="text-4xl text-mm-purple font-normal mb-16">{session.description}</h1>
                </div>
                <div class="flex flex-grow items-center px-80 {(backgroundVideoReady) ? '' : 'hidden'}">
                    <AdviceMessage advices={session.advices}/>
                </div>
            {/if}
        </div>
        {#if session}
            <div class="absolute bottom-0 z-10 w-full flex items-center justify-center">
                <div out:fade={{ duration: 1000 }}>
                    <VideoPlayer filename="{session.id}" class="max-h-[500px] h-auto"/>
                </div>
            </div>
        {/if}
        <div class="absolute bottom-14 right-14 z-20 text-gray-600 flex flex-col items-center">
            <div class="text-3xl mb-6">
                {#if countdownSeconds && countdownSeconds > 0}
                <span in:fade={{ duration: 1000 }}
                      out:fade={{ duration: 1000 }}>
                    {formatCountdown(countdownSeconds)}
                </span>
                {/if}
            </div>

            <button class="bg-white bg-opacity-5 hover:bg-white hover:bg-opacity-20 font-bold py-2 px-4 rounded-2xl border border-gray-700 inline-flex items-center"
                    on:click={closeApp}>
                {#if countdownSeconds && countdownSeconds > 0}
                    <Icon icon="material-symbols-light:fast-forward-outline-rounded" class="mr-2" height="32"/>
                    Skip
                {:else}
                    <Icon icon="material-symbols-light:check-circle-outline" class="mr-2" height="32"/>
                    Finished
                {/if}
            </button>
        </div>
    </div>

    <style>
        .video-background-ready {
            opacity: 1.0;
            transition: opacity 1s;
        }

        .video-not-ready {
            opacity: 0;
        }
    </style>
{/if}