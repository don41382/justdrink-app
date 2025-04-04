<script lang="ts">
    import {isEnabled, enable, disable} from "@tauri-apps/plugin-autostart";
    import {type AppDetails, commands, type Settings, type SettingsUserDetails} from '../../bindings';
    import {formatDuration, sessionTimes} from "../session-times";
    import {error, info} from "@tauri-apps/plugin-log";
    import {getCurrentWindow} from "@tauri-apps/api/window";

    export let user: SettingsUserDetails;
    export let updateSettings: (updatedSettings: SettingsUserDetails) => Promise<void>;

    let next_break_duration_minutes: number = user.next_break_duration_minutes;

    async function submit() {
        if (next_break_duration_minutes) {
            user.next_break_duration_minutes = next_break_duration_minutes;
        }

        const autoStartEnabled = await isEnabled()

        await info(`auto-start: ${autoStartEnabled}, user: ${user.enable_on_startup}`)

        if (autoStartEnabled != user.enable_on_startup) {
            if (user.enable_on_startup) {
                await enable()
            } else {
                await disable()
            }
        }
        await updateSettings(user);
    }

    function welcome_redo() {
        commands.welcomeRedo();
    }

</script>

<div class="flex-col space-y-6">
    <h2 class="text-lg font-semibold text-gray-900">Next Reminder</h2>
    <div class="space-y-2">
        <label class="flex justify-between items-center bg-white p-4 rounded-lg shadow-sm cursor-pointer">
            <span class="text-gray-700">Active</span>
            <input bind:checked={user.active} class="toggle-checkbox" on:change={submit} type="checkbox">
        </label>
        <label class="block justify-between items-center bg-white p-4 rounded-lg shadow-sm cursor-pointer">
            <div class="flex justify-between">
                <span class="text-gray-700">Next Drink Reminder</span>
                <select bind:value={next_break_duration_minutes}
                        class="p-2 border rounded-l shadow-sm text-right text-black w-24"
                        on:change={submit}>
                    {#each sessionTimes as duration}
                        <option value="{duration}">{formatDuration(duration)}</option>
                    {/each}
                </select>
            </div>

            <div class="flex w-full justify-between">
                <p class="text-gray-500 text-sm mt-1">Time until your next sip</p>
                <button class="text-gray-500 text-sm underline hover:text-accent mt-1 text-right cursor-pointer"
                        on:click={welcome_redo}>
                    Restart Wizard
                </button>
            </div>
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