<script lang="ts">
    import {error} from '@tauri-apps/plugin-log';
    import {commands, events} from '../bindings';
    import {Window} from '@tauri-apps/api/window';
    import {onDestroy, onMount} from 'svelte';
    import type {UnlistenFn} from '@tauri-apps/api/event';

    let sessionStartListener: UnlistenFn;

    onMount(async () => {
        sessionStartListener = await events.sessionStart.listen(async (e) => {
            try {
                const sessionWindow = new Window('session');
                await sessionWindow.show();
                await sessionWindow.setFocus();
            } catch (e) {
                if (e instanceof Error) {
                    await error(e.message);
                } else {
                    await error(String(e))
                }
            }
        });
    });

    onDestroy(() => {
        if (sessionStartListener) {
            sessionStartListener();
        }
    });

    function closeApp() {
        commands.closeApp()
    }


    // allows no context menu
    // document.addEventListener('contextmenu', event => event.preventDefault());
</script>

<div class="flex items-center justify-center h-screen gradient-background select-none cursor-default">
    <div class="text-center">
        <h1 class="text-4xl mb-4">It's time for your</h1>
        <h1 class="text-4xl mb-14"><b class="font-bold">Motion Minute</b></h1>

        <button on:click={closeApp}
                class="bg-blue-500 bg-opacity-50 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-2xl border border-white">
            Feeling much better
        </button>

    </div>
</div>