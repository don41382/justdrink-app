<script lang="ts">
    import {error, info} from '@tauri-apps/plugin-log';
    import {commands, events, type SessionDetail} from '../../bindings';
    import {Window} from '@tauri-apps/api/window';
    import {onDestroy, onMount} from 'svelte';
    import type {UnlistenFn} from '@tauri-apps/api/event';
    import {fade} from 'svelte/transition';
    import Icon from '@iconify/svelte';

    let sessionStartListenerUnregister: UnlistenFn;
    let session: SessionDetail | undefined = undefined;
    let countdownSeconds: number | undefined = undefined;
    let countdownInterval: number | undefined;

    info("Initialized Session Window")

    onMount(async () => {
        sessionStartListenerUnregister = await events.sessionStartEvent.listen(async ({payload}) => {
            try {
                await info("new session started");
                session = payload.details
                countdownSeconds = payload.details.duration_s

                const sessionWindow = new Window('session');
                await sessionWindow.show();
                await sessionWindow.setFocus();

                // Start the countdown
                countdownInterval = window.setInterval(() => {
                    if (countdownSeconds) {
                        countdownSeconds -= 1;
                        if (countdownSeconds <= 0) {
                            countdownSeconds = 0
                            clearInterval(countdownInterval);
                        }
                    }
                }, 1000);
            } catch (e) {
                if (e instanceof Error) {
                    await error(e.message);
                } else {
                    await error(String(e))
                }
            }
        });
    });

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

<div class="flex items-center justify-center h-screen gradient-background select-none cursor-default">
    <div class="text-center">
        {#if session !== undefined && countdownSeconds !== undefined}
            <h1 class="text-4xl mb-4">It's time for your</h1>
            <h1 class="text-4xl mb-14 font-bold">Motion Minute</h1>

            <h1 class="text-4xl mb-4" in:fade={{ delay: 100, duration: 1000 }}>{session.title}</h1>

            <div class="text-2xl mb-14">
                <span in:fade={{ delay: 100, duration: 1000 }}>{formatCountdown(countdownSeconds)}</span>
            </div>

            <button on:click={closeApp}
                    class="bg-white bg-opacity-5 hover:bg-white hover:bg-opacity-20 text-white font-bold py-2 px-4 rounded-2xl border border-gray-700 inline-flex items-center">
                {#if countdownSeconds > 0}
                    <Icon icon="material-symbols-light:fast-forward-outline-rounded" class="mr-2" height="32"/>
                    Skip
                {:else}
                    <Icon icon="mdi-light:home" class="mr-2" height="32"/>
                    Finished
                {/if}
            </button>
        {/if}

    </div>
</div>