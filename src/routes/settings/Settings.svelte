<script lang="ts">
    import { isEnabled, enable, disable } from "@tauri-apps/plugin-autostart";
    import type {Settings, SettingsUserDetails} from '../../bindings';
    import {formatDuration, sessionTimes} from "../session-times";
    import {info} from "@tauri-apps/plugin-log";

    export let user: SettingsUserDetails;
    export let updateSettings: (updatedSettings: SettingsUserDetails) => Promise<void>;

    let next_break_duration_minutes: number = user.next_break_duration_minutes;

    async function submit() {
        if (next_break_duration_minutes) {
            user.next_break_duration_minutes = next_break_duration_minutes;
        }
        if (await isEnabled() != user.enable_on_startup) {
            if (user.enable_on_startup) {
                await enable()
            } else {
                await disable()
            }
        }
        await updateSettings(user);
    }
</script>

<div class="space-y-6">
    <h2 class="text-lg font-semibold text-gray-900">Next Session</h2>
    <div class="mt-2 space-y-2">
        <label class="flex justify-between items-center bg-white p-4 rounded-lg shadow-sm cursor-pointer">
            <span class="text-gray-700">Active</span>
            <input bind:checked={user.active} class="toggle-checkbox" on:change={submit} type="checkbox">
        </label>
        <label class="flex justify-between items-center bg-white p-4 rounded-lg shadow-sm cursor-pointer">
            <span class="text-gray-700">Enable on startup</span>
            <input bind:checked={user.enable_on_startup} class="toggle-checkbox" on:change={submit} type="checkbox">
        </label>
        <label class="block justify-between items-center bg-white p-4 rounded-lg shadow-sm cursor-pointer">
            <div class="flex justify-between items-center">
                <span class="text-gray-700">Next session</span>
                <select bind:value={next_break_duration_minutes}
                        class="p-2 border rounded-l shadow-sm text-right text-black w-24"
                        on:change={submit}>
                    {#each sessionTimes as duration}
                        <option value="{duration}">{formatDuration(duration)}</option>
                    {/each}
                </select>
            </div>
            <p class="text-gray-500 text-sm mt-1">Time until the next session</p>
        </label>
    </div>
</div>