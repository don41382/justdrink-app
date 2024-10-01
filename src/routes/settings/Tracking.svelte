<script lang="ts">
    import {type SettingsUserDetails} from '../../bindings';

    export let user: SettingsUserDetails;

    export let updateSettings: (updatedSettings: SettingsUserDetails) => Promise<void>;

    async function submit() {
        await updateSettings(user)
    }
</script>

<div class="space-y-6">
    <h2 class="text-lg font-semibold text-gray-900">User Tracking Settings</h2>
    <div class="mt-2 space-y-2">
        <label class="block bg-white p-4 rounded-lg shadow-sm cursor-pointer">
            <div class="flex justify-between items-center">
                <span class="text-gray-700">Active</span>
                <input bind:checked={user.allow_tracking} class="toggle-checkbox" on:change={submit} type="checkbox">
            </div>
            <p class="text-gray-500 text-sm mt-1">All tracked information are anonymised.</p>
        </label>
    </div>
</div>