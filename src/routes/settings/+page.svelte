<script lang="ts">
    import {commands, events} from '../../bindings';
    import {onDestroy, onMount} from 'svelte';
    import type {UnlistenFn} from '@tauri-apps/api/event';
    import {Window} from "@tauri-apps/api/window";
    import {fade} from 'svelte/transition';
    import {info} from "@tauri-apps/plugin-log";


    let sessionStartListenerUnregister: UnlistenFn;
    let show: boolean = false;

    info("Initialized Setting Window")

    onMount(async () => {
        show = true
        await info("mounted settings window");
        sessionStartListenerUnregister = await events.settingsEvent.listen(async ({payload}) => {
            await info("show settings");
            const window = new Window('settings');
            await window.show();
            await window.setFocus();
            show = true;
        });
    });

    onDestroy(() => {
        if (sessionStartListenerUnregister) {
            sessionStartListenerUnregister();
        }
    });

    function closeApp() {
        show = false;
        commands.closeWindow();
    }

    // allows no context menu
    document.addEventListener('contextmenu', event => event.preventDefault());
</script>

{#if show}
    <div in:fade={{ duration: 500 }} class="cursor-default bg-gray-100 font-sans rounded-2xl h-full">
        <div class="flex">
            <!-- Sidebar -->
            <div class="w-64 bg-white border-r border-gray-200">
                <div class="p-4">
                    <div class="flex items-center space-x-2">
                        <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                        <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                        <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                    </div>
                </div>
                <div class="mt-4">
                    <ul>
                        <li class="px-4 py-2 bg-gray-200 text-gray-900 flex items-center space-x-2">
                            <i class="fas fa-cog"></i>
                            <span>General</span>
                        </li>
                        <li class="px-4 py-2 text-gray-700 flex items-center space-x-2">
                            <i class="fas fa-moon"></i>
                            <span>Focus mode</span>
                        </li>
                        <li class="px-4 py-2 text-gray-700 flex items-center space-x-2">
                            <i class="fas fa-bed"></i>
                            <span>Rest mode</span>
                        </li>
                        <li class="px-4 py-2 text-gray-700 flex items-center space-x-2">
                            <i class="fas fa-bell"></i>
                            <span>Notifications</span>
                        </li>
                        <li class="px-4 py-2 text-gray-700 flex items-center space-x-2">
                            <i class="fas fa-bars"></i>
                            <span>Menu bar</span>
                        </li>
                        <li class="px-4 py-2 text-gray-700 flex items-center space-x-2">
                            <i class="fas fa-keyboard"></i>
                            <span>Keyboard shortcuts</span>
                        </li>
                        <li class="px-4 py-2 text-gray-700 flex items-center space-x-2">
                            <i class="fas fa-info-circle"></i>
                            <span>About LookAway</span>
                        </li>
                    </ul>
                </div>
                <div class="absolute bottom-0 w-full p-4">
                    <button class="text-gray-700 flex items-center space-x-2">
                        <i class="fas fa-key"></i>
                        <span>Manage license</span>
                    </button>
                </div>
            </div>
            <!-- Main Content -->
            <div class="flex-1 p-8">
                <div class="space-y-6">
                    <!-- Startup Section -->
                    <div>
                        <h2 class="text-lg font-semibold text-gray-900">Startup</h2>
                        <div class="mt-2 space-y-2">
                            <div class="flex justify-between items-center bg-white p-4 rounded-lg shadow-sm">
                                <span class="text-gray-700">Launch at login</span>
                                <input type="checkbox" class="toggle-checkbox" checked>
                            </div>
                            <div class="flex justify-between items-center bg-white p-4 rounded-lg shadow-sm">
                                <span class="text-gray-700">Start timer automatically on launch</span>
                                <input type="checkbox" class="toggle-checkbox" checked>
                            </div>
                        </div>
                    </div>
                    <!-- Work Schedule Section -->
                    <div>
                        <h2 class="text-lg font-semibold text-gray-900">Work schedule</h2>
                        <div class="mt-2 space-y-2">
                            <div class="bg-white p-4 rounded-lg shadow-sm">
                                <div class="flex justify-between items-center">
                                    <span class="text-gray-700">Enable work schedule</span>
                                    <input type="checkbox" class="toggle-checkbox">
                                </div>
                                <p class="text-gray-500 text-sm mt-1">When enabled, LookAway will only show breaks
                                    during the set schedule</p>
                            </div>
                        </div>
                    </div>
                    <!-- Updates Section -->
                    <div>
                        <h2 class="text-lg font-semibold text-gray-900">Updates</h2>
                        <div class="mt-2 space-y-2">
                            <div class="flex justify-between items-center bg-white p-4 rounded-lg shadow-sm">
                                <span class="text-gray-700">Automatically check for updates</span>
                                <input type="checkbox" class="toggle-checkbox" checked>
                            </div>
                            <div class="flex justify-between items-center bg-white p-4 rounded-lg shadow-sm">
                                <span class="text-gray-700">Automatically download updates</span>
                                <input type="checkbox" class="toggle-checkbox">
                            </div>
                            <div class="bg-white p-4 rounded-lg shadow-sm">
                                <button class="bg-gray-200 text-gray-700 px-4 py-2 rounded">Check for updates</button>
                            </div>
                        </div>
                    </div>
                    <!-- Analytics Section -->
                    <div>
                        <h2 class="text-lg font-semibold text-gray-900">Analytics</h2>
                        <div class="mt-2 space-y-2">
                            <div class="bg-white p-4 rounded-lg shadow-sm">
                                <div class="flex justify-between items-center">
                                    <span class="text-gray-700">Share my usage statistics</span>
                                    <input type="checkbox" class="toggle-checkbox" checked>
                                </div>
                                <p class="text-gray-500 text-sm mt-1">Help us improve LookAway by allowing us to collect
                                    completely anonymous usage data</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}