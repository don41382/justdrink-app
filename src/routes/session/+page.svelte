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

    const fadeOutDurationSeconds = 2

    let exercise: Exercise | undefined = undefined;
    let license: LicenseInfo | undefined = undefined;

    let countdownSeconds: number | undefined;
    let countdownInterval: number | undefined;

    let backgroundLoadingFinished: boolean = false;

    let music: AudioPlayer;
    let finishSound: HTMLAudioElement;

    let show = true;

    function setup(details: SessionDetail) {
        license = details.license_info
        exercise = details.exercise
        countdownSeconds = details.exercise.duration_s

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

    async function init() {
        let resource_dir = await tauri_path.resourceDir();
        finishSound.src = convertFileSrc(`${resource_dir}/audio/session-end.mp3`)
        music.play();

        let sessionDetail = await commands.loadSessionDetails();
        if (sessionDetail == null || sessionDetail.license_info == null) {
            closeApp("Error")
        } else {
            setup(sessionDetail);
            const window = getCurrentWindow()
            await window.setFocus()
        }
    }

    onMount(async () => {
        await info("mount session window")
        await init().catch((err) => {
            error(`error during init: ${err}`);
        })
    });

    function cleanup() {
        if (countdownInterval) {
            clearInterval(countdownInterval);
        }
        exercise = undefined;
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
         class="{backgroundLoadingFinished ? 'video-background-ready' : 'video-not-ready'} bg-transparent h-screen flex flex-col justify-between items-center overflow-hidden cursor-default"
         out:fade={{duration: fadeOutDurationSeconds * 1000}} on:outrostart={beforeCloseApp}
         on:outroend={() => closeApp("EndOfTime")}>

        <AudioPlayer bind:this={music} filename="session-01.mp3" initialVolume={0.4}/>
        <audio bind:this={finishSound} src="" preload="auto"></audio>

        <BackgroundVideo bind:backgroundVideoLoaded={backgroundLoadingFinished}/>

        <div class="relative z-20 flex flex-col">
            {#if exercise !== undefined && countdownSeconds !== undefined}
                <div class="flex-none text-center mt-20 px-48">
                    <h1 class="text-8xl text-mm-blue font-bold mb-4">{exercise.title}</h1>
                    <h1 class="text-4xl text-mm-purple font-normal mb-16">{exercise.description}</h1>
                </div>
                <div class="flex flex-grow items-center px-80 {(backgroundLoadingFinished) ? '' : 'hidden'}">
                    <AdviceMessage advices={exercise.advices}/>
                </div>
            {/if}
        </div>
        {#if exercise}
            <div class="absolute bottom-0 z-10 w-full flex items-center justify-center">
                <div out:fade={{ duration: 1000 }}>
                    <VideoPlayer filename="{exercise.id}" class="max-h-[500px] h-auto"/>
                </div>
            </div>
        {/if}
        <div class="absolute top-14 right-14 z-20 text-gray-600 flex flex-col items-center">
            {#if license && license.message}
            <span class="text-black">{license.message}</span>
            {/if}
        </div>
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
                    on:click={() => closeApp("UserEscape")}>
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