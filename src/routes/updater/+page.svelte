<script lang="ts">
    import {info} from "@tauri-apps/plugin-log";
    import {commands} from "../../bindings";
    import {relaunch} from "@tauri-apps/plugin-process";
    import AutoSize from "../AutoSize.svelte";
    import type {UpdateState} from "./updateState";

    type ProgressStatus = "inactive" | "running" | "finished";

    let {data} = $props();
    let currentState: UpdateState = $state.raw(data.updateState);
    let progressStatus: ProgressStatus = $state("inactive")

    let downloaded = $state(0);
    let total = $state(0);

    let percentage = $derived(parseFloat((total > 0 ? (downloaded / total) * 100 : 0).toFixed(0)));

    async function closeWindow() {
        await commands.updaterClose();
    }


    async function installUpdate() {
        if (currentState.state === "updateAvailable") {
            let error = false;
            await currentState.update.downloadAndInstall(async (event) => {
                switch (event.event) {
                    case 'Started':
                        progressStatus = "running"
                        downloaded = 0;
                        total = event.data.contentLength ?? 0;
                        break;
                    case 'Progress':
                        downloaded += event.data.chunkLength;
                        break;
                    case 'Finished':
                        await info("finished download")
                        progressStatus = "finished"
                        downloaded = total
                        break;
                }
            }).catch((err) => {
                error = true;
                commands.alertLogClientError("Update Error", `Could not update Just Drink: ${err}`, `Error while updating: ${err}`);
            })
            setTimeout(async () => {
                if (!error) {
                    await info("relaunch after update")
                    await relaunch().catch(async (err) => {
                        await commands.alertLogClientError("Update Error", "Application relaunch failed. Please try restarting manually.", `Unable to relaunch: ${err}`);
                    })
                }
            }, 2000);
        }
    }

</script>

<AutoSize class="bg-white w-[500px] rounded-lg border-mm-blue-50 border-2 outline-mm-blue p-6" ready={true}>
    {#if currentState.state === "updateAvailable"}
        <!-- Logo and Update Info -->
        <div class="flex items-start mb-4">
            <img src="{data.iconPath}" alt="App Logo" class="w-14 h-14 mr-4">
            <div>
                <h1 class="text-xl font-semibold text-gray-800">Update Available</h1>
                <p class="text-gray-500">New version: <b>{currentState.update.version}</b></p>
            </div>
        </div>

        <p class="text-gray-600 mb-6">A new version of Just Drink! is available. Would you like to update to the
            latest version?</p>

        <!-- Progress bar (visible when download starts) -->
        {#if progressStatus === "running"}
            <div class="mb-4">
                <div class="w-full bg-gray-200 rounded-full h-2.5">
                    <div class="bg-blue-500 h-2.5 rounded-full" style="width: {percentage}%;"></div>
                </div>
                <p class="text-sm text-gray-500 mt-2">Updating... {percentage}%</p>
            </div>
        {/if}

        <div class="flex justify-end space-x-3">
            {#if progressStatus === "inactive"}
                <button class="bg-gray-300 text-gray-800 px-4 py-2 rounded-lg hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-300 cursor-pointer"
                        onclick={async () => closeWindow()}>
                    Later
                </button>
                <button class="bg-mm-green text-white px-4 py-2 rounded-lg hover:bg-mm-green-600 focus:outline-none focus:ring-2 focus:ring-blue-500 cursor-pointer"
                        onclick={async () => installUpdate()}>
                    Update Now
                </button>
            {:else if progressStatus === "running"}
                <p>Updating ...</p>
            {:else if progressStatus === "finished"}
                <p>Please wait while application restarts ....</p>
            {/if}
        </div>
    {:else if currentState.state === "upToDate"}
        <!-- Logo and Update Info -->
        <div class="flex items-start mb-4">
            <img src="{data.iconPath}" alt="App Logo" class="w-14 h-14 mr-4">
            <div>
                <h1 class="text-xl font-semibold text-gray-800">Up to date.</h1>
                <p class="text-gray-500">There is no newer version.</p>
            </div>
        </div>

        <p class="text-gray-600 mb-6">You are currently on the latest version. Please check later.</p>

        <div class="flex justify-end space-x-3">
            <button class="bg-gray-300 text-gray-800 px-4 py-2 rounded-lg hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-300 cursor-pointer"
                    onclick={async () => closeWindow()}>
                Close
            </button>
        </div>
    {:else}
        <h1 class="text-xl font-normal text-amber-950 mb-4">Unknown state.</h1>
    {/if}
</AutoSize>
