<script lang="ts">
    import {commands, events, type Settings} from '../../bindings';
    import {onDestroy, onMount} from 'svelte';
    import type {UnlistenFn} from '@tauri-apps/api/event';
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {info} from "@tauri-apps/plugin-log";
    import Icon from '@iconify/svelte';

    let sessionStartListenerUnregister: UnlistenFn;
    let closeRequestUnregister: UnlistenFn;

    let settings: Settings | undefined = undefined;


    onMount(async () => {
        await info("mounted settings window");
        sessionStartListenerUnregister = await events.settingsEvent.listen(async ({payload}) => {
            await info("show settings");

            settings = payload.details
            const window = getCurrentWindow();
            await window.show();
            await window.setFocus();
        });
        closeRequestUnregister = await getCurrentWindow().listen("tauri://close-requested", (e) => {
            info("close settings window")
        })
    });

    onDestroy(() => {
        if (sessionStartListenerUnregister) {
            sessionStartListenerUnregister();
        }
        if (closeRequestUnregister) {
            closeRequestUnregister();
        }
    });

    function submit() {
        if (settings) {
            info(`submit: ${settings.active}`);
            commands.updateSettings(settings);
        }
    }


    // allows no context menu
    document.addEventListener('contextmenu', event => event.preventDefault());
</script>

{#if settings}
    <div class="cursor-default bg-gray-100 font-sans rounded-b-2xl h-screen flex overflow-hidden">
        <!-- Sidebar -->
        <div class="w-64 bg-white border-r border-gray-200 flex flex-col">
            <div class="flex-grow overflow-y-auto">
                <ul>
                    <li class="px-6 py-4 bg-gray-200 text-gray-900 flex items-center space-x-2">
                        <Icon icon="mdi-light:settings" height="24"/>
                        <span>Session</span>
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
            <div class="p-8 space-y-6">
                <!-- Startup Section -->
                <div>
                    <h2 class="text-lg font-semibold text-gray-900">Next Session</h2>
                    <div class="mt-2 space-y-2">
                        <label class="flex justify-between items-center bg-white p-4 rounded-lg shadow-sm cursor-pointer">
                            <span class="text-gray-700">Active</span><input bind:checked={settings.active}
                                                                            on:change={submit} type="checkbox"
                                                                            class="toggle-checkbox"></label>
                        <label class="block justify-between items-center bg-white p-4 rounded-lg shadow-sm cursor-pointer">
                            <div class="flex justify-between items-center">
                                <span class="text-gray-700">Next session</span>
                                <input type="number"
                                       class="p-2 border rounded-l shadow-sm text-right placeholder-right text-black w-24"
                                       maxlength="2" pattern="\d{1,2}" required inputmode="numeric" placeholder="min"
                                       on:change={submit}
                                       bind:value="{settings.next_break_duration_minutes}">
                            </div>
                            <p class="text-gray-500 text-sm mt-1">Minutes until next session</p>

                        </label>
                    </div>
                </div>
                <!-- Work Schedule Section -->
                <div>
                    <h2 class="text-lg font-semibold text-gray-900">Work schedule</h2>
                    <div class="mt-2 space-y-2">
                        <label class="block bg-white p-4 rounded-lg shadow-sm cursor-pointer">
                            <div class="flex justify-between items-center">
                                <span class="text-gray-700">Enable work schedule</span>
                                <input type="checkbox" class="toggle-checkbox" disabled>
                            </div>
                            <p class="text-gray-500 text-sm mt-1">When enabled, LookAway will only show breaks during
                                the set schedule</p>
                        </label>
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    .toggle-checkbox {
        appearance: none;
        width: 2.5rem;
        height: 1.25rem;
        background-color: #d1d5db;
        border-radius: 9999px;
        position: relative;
        cursor: pointer;
        outline: none;
        transition: background-color 0.2s;
    }

    .toggle-checkbox:checked {
        background-color: #3b82f6;
    }

    .toggle-checkbox:checked::before {
        transform: translateX(1.25rem);
    }

    .toggle-checkbox::before {
        content: '';
        position: absolute;
        top: 0.125rem;
        left: 0.125rem;
        width: 1rem;
        height: 1rem;
        background-color: #fff;
        border-radius: 9999px;
        transition: transform 0.2s;
    }
</style>