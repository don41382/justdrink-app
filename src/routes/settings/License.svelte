<script lang="ts">
    import {
        type AppDetails,
        commands,
    } from '../../bindings';
    import {info} from "@tauri-apps/plugin-log";

    export let app: AppDetails;

    let licenseCode: string | null = app.license_info.license_key;
    let error: string | null = null;

    function formatCode(value: string) {
        // Remove all non-alphanumeric characters and convert to uppercase
        value = value.toUpperCase().replace(/[^A-Z0-9]/g, '');

        let formattedValue = '';
        let chunkSizes = [5, 5, 5, 5, 2]; // Define the chunk sizes for the format 5-5-5-5-2
        let currentIndex = 0;

        // Loop through the chunk sizes and apply them to the input value
        for (let i = 0; i < chunkSizes.length; i++) {
            if (currentIndex >= value.length) {
                break; // Stop if we've processed the entire value
            }

            // Get the substring for the current chunk
            let chunk = value.substring(currentIndex, currentIndex + chunkSizes[i]);
            if (formattedValue) {
                formattedValue += '-'; // Add a dash between chunks
            }
            formattedValue += chunk;

            currentIndex += chunkSizes[i]; // Move the index forward by the chunk size
        }

        return formattedValue;
    }


    function handleInput(event: InputEvent) {
        if (event.target instanceof HTMLInputElement) {
            licenseCode = formatCode(event.target.value);
        }
    }

    async function registerLicense() {
        await info("submit license with key: " + licenseCode)
        if (licenseCode) {
            let result = await commands.settingsRegisterLicense(licenseCode);
            switch (result.status) {
                case "Trail":
                    app.license_info = result;
                    error = null;
                    break;
                case "Paid":
                    app.license_info = result;
                    error = null;
                    break;
                case "Invalid":
                    error = result.message;
                    break;

            }
        } else {
            error = "Please enter a valid license key"
        }
    }

    async function reset() {
        await info("reset license")
        app.license_info = await commands.settingsResetLicense();
    }

    async function getALicense() {
        await info("get a license")
        await commands.getALicense()
    }

</script>

<div class="space-y-6">
    <div class="flex justify-between items-center">
        <h2 class="text-lg font-semibold text-gray-900">License</h2>
        <div class="flex items-center rounded-full px-3 py-1 text-sm bg-gray-200 {app.license_info.status === 'Trail' || app.license_info.status === 'Paid'  ? 'text-black' : 'text-mm-orange'}">
            {#if app.license_info.message}
                {app.license_info.message}
            {/if}
        </div>
    </div>
    <div class="flex-col">
        {#if app.license_info.status === 'Paid'}
            <div class="flex-col">
                <p class="text-mm-green font-normal accent-mm-green">Your license is active.</p>
                <p class="text-gray-700 font-thin mb-4">Thank you for supporting Motion Minute.</p>
            </div>
        {:else}
            <p class="text-gray-700 mb-4">
                Enter the license key you received in your email after purchasing Motion Minute to activate it on this
                device:
            </p>
        {/if}
        <input bind:value={licenseCode}
               class="w-full border-2 border-gray-300 rounded-lg p-2 mb-1 focus:outline-none focus:border-blue-500 disabled:opacity-50"
               disabled={app.license_info.license_key === licenseCode && app.license_info.status === 'Paid'}
               maxlength="29" on:input={handleInput}
               placeholder="XXXXX-XXXXX-XXXXX-XXXXX-XX" type="text"/>
        <div class="mb-4">
            {#if error}
                <p class="text-mm-orange text-sm">{error}</p>
            {/if}
        </div>
        <div class="flex justify-between">
            {#if app.license_info.status === 'Paid' || app.license_info.status === 'Invalid'}
                <button class="text-white rounded-lg px-4 py-2 bg-gray-500 hover:bg-gray-800 ml-auto"
                        on:click={async () => reset()}>
                    Reset license
                </button>
            {:else}
                <button class="bg-white border border-gray-300 text-gray-700 rounded-lg px-4 py-2" on:click={async () => getALicense()}>
                    Get a license
                </button>
                <button class="text-white rounded-lg px-4 py-2 bg-mm-orange hover:bg-mm-orange-200"
                        on:click={async () => registerLicense()}>
                    Activate Motion Minute
                </button>
            {/if}
        </div>
    </div>
</div>