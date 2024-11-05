<script lang="ts">
    import {check, Update} from '@tauri-apps/plugin-updater';
    import {afterUpdate, onMount} from "svelte";
    import {info} from "@tauri-apps/plugin-log";
    import {commands} from "../../bindings";
    import * as tauri_path from "@tauri-apps/api/path";
    import {convertFileSrc} from "@tauri-apps/api/core";
    import {fitAndShowWindow} from "../../helper";
    import {relaunch} from "@tauri-apps/plugin-process";

    let contentDiv: HTMLDivElement;

    type State = "init" | "started" | "progress" | "finished" | "newest" | "error"

    let downloaded: number = 0
    let total: number = 0
    $: percentage = parseFloat((total > 0 ? (downloaded / total) * 100 : 0).toFixed(0));

    let error: string | undefined;

    let state: State = "init"
    let icon_path: string;

    let update: Update | null = null;

    async function closeWindow() {
        await commands.updaterClose();
    }

    onMount(async () => {
        await info("mount update window")
        let resource_dir = await tauri_path.resourceDir();
        icon_path = convertFileSrc(`${resource_dir}/icons/128x128.png`);

        update = await check().then(async (res) => {
            if (res == null) {
                state = "newest"
            }
            return res;
        }).catch(async (e) => {
            await info(`error while checking for update: ${e}`)
            await fitAndShowWindow(contentDiv);
            error = `Error while checking for update: ${e}`
            state = "error"
            return null;
        });
    });

    afterUpdate(async () => {
        await fitAndShowWindow(contentDiv);
    });

    async function installUpdate() {
        if (update) {
            await update.downloadAndInstall(async (event) => {
                switch (event.event) {
                    case 'Started':
                        state = "started"
                        downloaded = 0;
                        total = event.data.contentLength ?? 0;
                        break;
                    case 'Progress':
                        state = "progress"
                        downloaded += event.data.chunkLength;
                        break;
                    case 'Finished':
                        state = "finished"
                        downloaded = total
                        break;
                }
            });
            await relaunch()
        }
    }

</script>

<div bind:this={contentDiv} class="bg-white rounded-lg border-mm-blue-50 border-2 outline-mm-blue w-full p-6">
    {#if update}
        <!-- Logo and Update Info -->
        <div class="flex items-start mb-4">
            <img src="{icon_path}" alt="App Logo" class="w-14 h-14 mr-4">
            <div>
                <h1 class="text-xl font-semibold text-gray-800">Update Available</h1>
                <p class="text-gray-500">New version: <b>{update.version}</b></p>
            </div>
        </div>

        <p class="text-gray-600 mb-6">A new version of Motion Minute is available. Would you like to update to the
            latest version?</p>

        <!-- Progress bar (visible when download starts) -->
        {#if state === "started" || state === "progress"}
            <div class="mb-4">
                <div class="w-full bg-gray-200 rounded-full h-2.5">
                    <div class="bg-blue-500 h-2.5 rounded-full" style="width: {percentage}%;"></div>
                </div>
                <p class="text-sm text-gray-500 mt-2">Updating... {percentage}%</p>
            </div>
        {/if}

        <div class="flex justify-end space-x-3">
            {#if state === "init"}
                <button class="bg-gray-300 text-gray-800 px-4 py-2 rounded-lg hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-300 cursor-pointer"
                        on:click={async () => closeWindow()}>
                    Later
                </button>
                <button class="bg-mm-green text-white px-4 py-2 rounded-lg hover:bg-mm-green-600 focus:outline-none focus:ring-2 focus:ring-blue-500 cursor-pointer"
                        on:click={async () => installUpdate()}>
                    Update Now
                </button>
            {/if}
        </div>
    {:else}
        {#if state === "newest" }
            <!-- Logo and Update Info -->
            <div class="flex items-start mb-4">
                <img src="{icon_path}" alt="App Logo" class="w-14 h-14 mr-4">
                <div>
                    <h1 class="text-xl font-semibold text-gray-800">Up to date.</h1>
                    <p class="text-gray-500">Their is no newer version.</p>
                </div>
            </div>

            <p class="text-gray-600 mb-6">You are currently on the newest version. Please check later.</p>

            <div class="flex justify-end space-x-3">
                <button class="bg-gray-300 text-gray-800 px-4 py-2 rounded-lg hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-300 cursor-pointer"
                        on:click={async () => closeWindow(false)}>
                    Close
                </button>
            </div>
        {:else if state === "error"}
            <h1 class="text-xl font-normal text-amber-950 mb-4">Error on Update.</h1>
            <p class="text-gray-600 mb-6">{error}</p>
            <div class="flex justify-end space-x-3">
                <button class="bg-gray-300 text-gray-800 px-4 py-2 rounded-lg hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-300 cursor-pointer"
                        on:click={async () => closeWindow(false)}>
                    Close
                </button>
            </div>
        {:else}
            <h1 class="font-normal text-gray-800">Please wait, checking version ...</h1>
        {/if}
    {/if}
</div>
