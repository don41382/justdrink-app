<script lang="ts">
    import {commands, events, type Settings, type SettingsUserDetails, type SettingsTabs} from '../../bindings';
    import {onDestroy, onMount} from 'svelte';
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {debug, info} from "@tauri-apps/plugin-log";
    import Icon from "@iconify/svelte";
    import Session from "./Settings.svelte";
    import Tracking from "./Tracking.svelte";
    import About from "./About.svelte";
    import License from "./License.svelte";
    import AutoSize from "../AutoSize.svelte";

    let { data } = $props()
    let settings = $state(data.settings)

    const params = new URLSearchParams(window.location.search);

    let currentPage: SettingsTabs =  $state(toSettingsTab(params.get("settings_tab")));
    let ready = $state(false);

    type Page = {
        name: SettingsTabs;
        label: string;
        icon: string;
    };

    const pages: Page[] = [
        {name: 'Session', label: 'Session', icon: 'mdi-light:settings'},
        {name: 'Tracking', label: 'User Tracking', icon: 'material-symbols-light:timeline'},
        {name: 'License', label: 'License', icon: 'material-symbols-light:license-outline'},
        {name: 'About', label: 'About', icon: 'material-symbols-light:info-outline'}
    ];

    async function updateSettings(updatedSettings: SettingsUserDetails) {
        if (settings) {
            settings.user = updatedSettings;
            await commands.updateSettings(updatedSettings);
        }
    }

    onMount(async () => {
        ready = true;
    })

    function toSettingsTab(tab: string | null): SettingsTabs {
        if (tab == null) {
            return 'Session'
        } else if (tab === 'Session' || tab === 'Tracking' || tab === 'License' || tab === 'About') {
            return tab;
        } else {
            throw new Error(`Invalid SettingsTab: ${tab}`);
        }
    }

    // allows no context menu
    document.addEventListener('contextmenu', event => event.preventDefault());
</script>

<AutoSize class="cursor-default w-[800px] h-[500px] bg-gray-100 rounded-b-2xl flex overflow-x-hidden" ready={ready}>
    {#if settings}
        <!-- Sidebar -->
        <div class="w-64 bg-white border-r border-gray-200 flex flex-col">
            <div class="flex-grow overflow-y-auto pt-8 px-4">
                <ul class="space-y-2">
                    {#each pages as page}
                        <li>
                            <button
                                    class="flex w-full items-center {currentPage === page.name ? 'text-white bg-primary' : 'text-black'} rounded-lg py-2 px-4 cursor-pointer"
                                    onclick={() => currentPage = page.name}>
                                <Icon class="mr-2" icon={page.icon} height="24" width="24"/>
                                <span>{page.label}</span>
                            </button>
                        </li>
                    {/each}
                </ul>
            </div>
        </div>
        <!-- Main Content -->
        <div class="flex-1 overflow-y-auto p-8">
            {#if currentPage === 'Session'}
                <Session user={settings.user} {updateSettings}/>
            {:else if currentPage === 'Tracking'}
                <Tracking user={settings.user} {updateSettings}/>
            {:else if currentPage === 'License'}
                <License app={settings.app}/>
            {:else if currentPage === 'About'}
                <About app={settings.app}/>
            {/if}
        </div>
    {:else}
        <h2>Loading ...</h2>
    {/if}
</AutoSize>