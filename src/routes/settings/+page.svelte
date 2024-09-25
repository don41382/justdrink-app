<script lang="ts">
    import {commands, events, type SettingsDetails} from '../../bindings';
    import {onDestroy, onMount} from 'svelte';
    import type {UnlistenFn} from '@tauri-apps/api/event';
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {info} from "@tauri-apps/plugin-log";
    import Icon from "@iconify/svelte";
    import Session from "./Settings.svelte";
    import Tracking from "./Tracking.svelte";

    let closeRequestUnregister: UnlistenFn;
    let settings: SettingsDetails | undefined = undefined;
    let currentPage: 'session' | 'tracking' = 'session';

    onMount(async () => {
        await info("mounted settings");
        settings = await commands.loadSettingsDetails();
        const window = getCurrentWindow();
        await window.show();
        await window.setFocus();
    });

    onDestroy(() => {
        if (closeRequestUnregister) {
            closeRequestUnregister();
        }
    });

    async function updateSettings(updatedSettings: SettingsDetails) {
        settings = updatedSettings;
        await commands.updateSettings(settings);
    }

    // allows no context menu
    document.addEventListener('contextmenu', event => event.preventDefault());
</script>

<div class="cursor-default bg-gray-100 font-sans rounded-b-2xl h-screen flex overflow-hidden">
    {#if settings}
        <!-- Sidebar -->
        <div class="w-64 bg-white border-r border-gray-200 flex flex-col">
            <div class="flex-grow overflow-y-auto">
                <ul>
                    <li>
                        <button
                                class="px-6 py-4 w-full text-left {currentPage === 'session' ? 'bg-gray-200' : ''} text-gray-900 flex items-center space-x-2 cursor-pointer"
                                on:click={() => currentPage = 'session'}
                        >
                            <Icon icon="mdi-light:settings" height="24"/>
                            <span>Session</span>
                        </button>
                    </li>
                    <li>
                        <button
                                class="px-6 py-4 w-full text-left {currentPage === 'tracking' ? 'bg-gray-200' : ''} text-gray-900 flex items-center space-x-2 cursor-pointer"
                                on:click={() => currentPage = 'tracking'}
                        >
                            <Icon icon="mdi-light:settings" height="24"/>
                            <span>User Tracking</span>
                        </button>
                    </li>
                </ul>
            </div>
            <div class="p-4">
                <button class="text-gray-700 flex items-center space-x-2">
                    <i class="fas fa-key"></i>
                    <span>Manage license</span>
                </button>
            </div>
        </div>
        <!-- Main Content -->
        <div class="flex-1 overflow-y-auto">
            {#if currentPage === 'session'}
                <Session {settings} {updateSettings}/>
            {:else if currentPage === 'tracking'}
                <Tracking {settings} {updateSettings}/>
            {/if}
        </div>
    {/if}
</div>