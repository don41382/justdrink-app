<script lang="ts">
    import { isEnabled, enable, disable } from "@tauri-apps/plugin-autostart";
    import type {Settings, SettingsUserDetails} from '../../bindings';

    export let user: SettingsUserDetails;
    export let updateSettings: (updatedSettings: SettingsUserDetails) => Promise<void>;

    let next_break_duration_minutes: string = user.next_break_duration_minutes.toString();

    async function submit() {
        if (next_break_duration_minutes) {
            user.next_break_duration_minutes = parseInt(next_break_duration_minutes);
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
                    <option value="10">10 min</option>
                    <option value="30">30 min</option>
                    <option value="60">1 hour</option>
                    <option value="120">2 hours</option>
                    <option value="180">3 hours</option>
                </select>
            </div>
            <p class="text-gray-500 text-sm mt-1">Time until the next session</p>
        </label>
    </div>
</div>