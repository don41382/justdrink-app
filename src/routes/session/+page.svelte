<script lang="ts">
    import {info} from '@tauri-apps/plugin-log';
    import {commands, type SessionDetail} from '../../bindings';
    import {getCurrentWindow} from '@tauri-apps/api/window';
    import {onDestroy, onMount} from 'svelte';
    import {fade} from 'svelte/transition';
    import Icon from '@iconify/svelte';
    import AdviceMessage from "./AdviceMessage.svelte";
    import VideoPlayer from "./VideoPlayer.svelte";

    let session: SessionDetail | undefined = undefined;
    let countdownSeconds: number | undefined;
    let countdownInterval: number | undefined;

    let backgroundVideo: HTMLVideoElement;
    let backgroundVideoReady = false;

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
            }
        }, 1000);
    }

    onMount(async () => {
        setup(await commands.loadSessionDetails());

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

<div aria-pressed="true"
     class="{backgroundVideoReady ? 'video-background-ready' : 'video-not-ready'} bg-transparent h-screen flex flex-col justify-between items-center overflow-hidden {backgroundVideoReady ? 'video-background-ready' : 'video-not-ready'} cursor-default"
     in:fade={{duration: 1000}}>
    <!-- background video -->
    <video
            autoplay
            bind:this={backgroundVideo}
            class="absolute w-full h-full object-cover blur-sm"
            loop muted on:canplay={setBackgroundVideoReady} playsinline
            preload="auto">
        <source src="/videos/bg-h264.mov" type="video/mp4">
        Your browser does not support the video tag.
    </video>

    <div class="relative z-10 flex flex-col h-full">
        {#if session !== undefined && countdownSeconds !== undefined}
            <div class="flex-none text-center mt-20 px-48">
                <h1 class="text-8xl text-mm-blue font-bold mb-4">{session.title}</h1>
                <h1 class="text-4xl text-mm-purple font-normal mb-16">{session.description}</h1>
            </div>
            <div class="flex flex-grow items-center w-full px-48 {(backgroundVideoReady) ? '' : 'hidden'}">
                <AdviceMessage advices={session.advices}/>
            </div>
            <div class="flex-none w-full flex items-center justify-center">
                <div out:fade={{ duration: 1000 }}>
                    <VideoPlayer filename="{session.id}/{session.id}" class="max-h-[500px]"/>
                </div>
            </div>
        {/if}
    </div>
    <div class="absolute bottom-14 right-14 z-50 text-gray-600 flex flex-col items-center">
        <div class="text-3xl mb-6">
            {#if countdownSeconds && countdownSeconds > 0}
                <span in:fade={{ delay: 100, duration: 1000 }}
                      out:fade={{ delay: 100, duration: 1000 }}>
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