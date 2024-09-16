<script lang="ts">
    import {error, info} from '@tauri-apps/plugin-log';
    import {commands, events, type SessionDetail} from '../../bindings';
    import {getCurrentWindow} from '@tauri-apps/api/window';
    import {onDestroy, onMount} from 'svelte';
    import type {UnlistenFn} from '@tauri-apps/api/event';
    import {platform} from '@tauri-apps/plugin-os';
    import {fade} from 'svelte/transition';
    import Icon from '@iconify/svelte';

    let sessionStartListenerUnregister: UnlistenFn;
    let session: SessionDetail | undefined = undefined;
    let countdownSeconds: number | undefined = undefined;
    let countdownInterval: number | undefined;

    let backgroundVideo: HTMLVideoElement;
    let backgroundVideoReady = false;

    let teacherVideo: HTMLVideoElement;
    let teacherVideoReady = false;

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

        sessionStartListenerUnregister = await events.sessionStartEvent.listen(async ({payload}) => {
            try {
                await info("new session started")

                if (!session) {
                    setup(payload.details)
                }

                const sessionWindow = getCurrentWindow()
                await sessionWindow.show()

                await sessionWindow.setFocus()

            } catch (e) {
                if (e instanceof Error) {
                    await error(e.message);
                } else {
                    await error(String(e))
                }
            }
        });

    });

    function setTeacherVideoReady() {
        if (teacherVideo.readyState === 4) {
            teacherVideoReady = true;
        }
    }

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
        if (sessionStartListenerUnregister) {
            sessionStartListenerUnregister();
        }
        cleanup();
    });

    function closeApp() {
        cleanup();
        commands.closeWindow();
    }


    function formatCountdown(seconds: number): string {
        const minutes = Math.floor(seconds / 60);
        const secs = seconds % 60;
        return `${String(minutes).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
    }


    // allows no context menu
    // document.addEventListener('contextmenu', event => event.preventDefault());
</script>

<div class="bg-white h-screen flex flex-col justify-between items-center overflow-hidden">

    <!-- background video -->
    <video
            bind:this={backgroundVideo}
            on:canplay={setBackgroundVideoReady}
            class="absolute w-full h-full object-cover blur-sm {backgroundVideoReady ? 'video-background-ready' : 'video-not-ready'}"
            autoplay loop muted playsinline
            preload="auto">
        <source src="/videos/bg-h264.mov" type="video/mp4">
        Your browser does not support the video tag.
    </video>

    <div class="relative z-10 flex flex-col justify-between items-center h-full">
        {#if session !== undefined && countdownSeconds !== undefined}
            <div class="text-center mt-24 px-48 z-10">
                <h1 class="text-8xl mb-8">{session.title}</h1>
                <p class="text-4xl font-thin leading-normal text-black">{session.subtitle}</p>
            </div>
            <div class="flex flex-col items-center w-full">
                <video
                        bind:this={teacherVideo}
                        on:canplay={setTeacherVideoReady}
                        class="w-full max-w-[500px] {teacherVideoReady ? 'video-teacher-ready' : 'video-not-ready'}"
                        autoplay loop muted playsinline preload="metadata">
                    <source
                            src="/videos/shoulder-hvc1.mov"
                            type='video/mp4; codecs="hvc1"'>
                    <source
                            src="/videos/shoulder.webm"
                            type="video/webm">
                    Your browser does not support the video tag.
                </video>
            </div>

            <div class="absolute bottom-14 right-14 text-gray-600 flex flex-col items-center">
                <div class="text-3xl mb-6">
                    <span in:fade={{ delay: 100, duration: 1000 }}>{formatCountdown(countdownSeconds)}</span>
                </div>

                <button on:click={closeApp}
                        class="bg-white bg-opacity-5 hover:bg-white hover:bg-opacity-20 font-bold py-2 px-4 rounded-2xl border border-gray-700 inline-flex items-center">
                    {#if countdownSeconds > 0}
                        <Icon icon="material-symbols-light:fast-forward-outline-rounded" class="mr-2" height="32"/>
                        Skip
                    {:else}
                        <Icon icon="material-symbols-light:check-circle-outline" class="mr-2" height="32"/>
                        Finished
                    {/if}
                </button>
            </div>
        {/if}

    </div>
</div>

<style>
    .video-teacher-ready {
        opacity: 1.0;
        transition: opacity 4s;
    }

    .video-background-ready {
        opacity: 0.8;
        transition: opacity 1s;
    }

    .video-not-ready {
        opacity: 0;
    }
</style>