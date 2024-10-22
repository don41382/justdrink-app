<script lang="ts">
    import {commands, events, type Settings, type SettingsUserDetails} from '../../bindings';
    import {onDestroy, onMount} from 'svelte';
    import type {UnlistenFn} from '@tauri-apps/api/event';
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {debug, info} from "@tauri-apps/plugin-log";
    import Icon from "@iconify/svelte";
    import Session from "./Settings.svelte";
    import Tracking from "./Tracking.svelte";
    import About from "./About.svelte";
    import {getCurrentWebviewWindow} from "@tauri-apps/api/webviewWindow";
    import License from "./License.svelte";

    let settings: Settings | undefined = undefined;

    type SettingsPage = 'session' | 'tracking' | 'about';
    let currentPage: SettingsPage = 'session';

    type Page = {
        name: SettingsPage;
        label: string;
        icon: string;
    };

    const pages: Page[] = [
        {name: 'session', label: 'Session', icon: 'mdi-light:settings'},
        {name: 'tracking', label: 'User Tracking', icon: 'material-symbols-light:timeline'},
        {name: 'license', label: 'License', icon: 'material-symbols-light:license-outline'},
        {name: 'about', label: 'About', icon: 'material-symbols-light:info-outline'}
    ];


    onMount(async () => {
        await info("mount settings window")

        await events.settingsStartEvent.once(async ({payload}) => {
            if (payload.start_with_about) {
                currentPage = 'about';
            }
        })

        settings = await commands.loadSettings();
        const window = getCurrentWindow();
        await window.show();
        await window.setFocus();
    });

    async function updateSettings(updatedSettings: SettingsUserDetails) {
        if (settings) {
            settings.user = updatedSettings;
            await commands.updateSettings(updatedSettings);
        }
    }

    // allows no context menu
    document.addEventListener('contextmenu', event => event.preventDefault());
</script>

<div class="cursor-default bg-gray-100 font-sans rounded-b-2xl h-screen flex overflow-hidden">
    {#if settings}
        <!-- Sidebar -->
        <div class="w-64 bg-white border-r border-gray-200 flex flex-col">
            <div class="flex-grow overflow-y-auto pt-8 px-4">
                <ul class="space-y-2">
                    {#each pages as page}
                        <li>
                            <button
                                    class="flex w-full items-center {currentPage === page.name ? 'text-white bg-mm-green' : 'text-black'} rounded-lg py-2 px-4 cursor-pointer"
                                    on:click={() => currentPage = page.name}>
                                <Icon class="mr-2" icon="{page.icon}" height="24" width="24"/>
                                <span>{page.label}</span>
                            </button>
                        </li>
                    {/each}
                </ul>
            </div>
            <!--
            <div class="p-4">
                <button class="text-gray-700 flex items-center space-x-2">
                    <i class="fas fa-key"></i>
                    <span>Manage license</span>
                </button>
            </div>
            -->
        </div>
        <!-- Main Content -->
        <div class="flex-1 overflow-y-auto p-8">
            {#if currentPage === 'session'}
                <Session user={settings.user} {updateSettings}/>
            {:else if currentPage === 'tracking'}
                <Tracking user={settings.user} {updateSettings}/>
            {:else if currentPage === 'license'}
                <License app={settings.app}/>
            {:else if currentPage === 'about'}
                <About app={settings.app}/>
            {/if}
        </div>
    {:else}
        <h2>Loading ...</h2>
    {/if}
</div>