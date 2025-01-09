<script lang="ts">
    import { isEnabled, enable, disable } from "@tauri-apps/plugin-autostart";
    import type {Settings, SettingsUserDetails} from '../../bindings';
    import {formatDuration, sessionTimes} from "../session-times";
    import {info} from "@tauri-apps/plugin-log";

    export let user: SettingsUserDetails;
    export let updateSettings: (updatedSettings: SettingsUserDetails) => Promise<void>;

    info(`settings-beta: ${user.beta_version}`)

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

<div class="flex-col space-y-6">
    <h2 class="text-lg font-semibold text-gray-900">Next Session</h2>
    <div class="space-y-2">
        <label class="flex justify-between items-center bg-white p-4 rounded-lg shadow-sm cursor-pointer">
            <span class="text-gray-700">Active</span>
            <input bind:checked={user.active} class="toggle-checkbox" on:change={submit} type="checkbox">
        </label>
        <label class="block justify-between items-center bg-white p-4 rounded-lg shadow-sm cursor-pointer">
            <div class="flex justify-between items-center">
                <span class="text-gray-700">Next Motion</span>
                <select bind:value={next_break_duration_minutes}
                        class="p-2 border rounded-l shadow-sm text-right text-black w-24"
                        on:change={submit}>
                    {#each sessionTimes as duration}
                        <option value="{duration}">{formatDuration(duration)}</option>
                    {/each}
                </select>
            </div>
            <p class="text-gray-500 text-sm mt-1">Time until the next motion</p>
        </label>
    </div>
    <h2 class="text-lg font-semibold text-gray-900">Advanced Settings</h2>
    <div class="space-y-2">
        <label class="flex justify-between items-center bg-white p-4 rounded-lg shadow-sm cursor-pointer">
            <span class="text-gray-700">Enable on startup</span>
            <input bind:checked={user.enable_on_startup} class="toggle-checkbox" on:change={submit} type="checkbox">
        </label>
        <label class="block justify-between items-center bg-white p-4 rounded-lg shadow-sm cursor-pointer">
            <div class="flex justify-between items-center">
                <span class="{user.beta_version ? 'text-gray-700' : 'text-gray-400' }">Enable Beta Access</span>
                <input bind:checked={user.beta_version} class="toggle-checkbox" on:change={submit} type="checkbox">
            </div>
            <div class="{user.beta_version ? 'text-gray-500' : 'text-gray-400' } text-sm space-y-1 mt-1">
                <p> Opt in to receive early access to beta versions.</p>
                <p> Please note that these versions may contain bugs or be unstable.</p>
            </div>
        </label>
    </div>
</div>