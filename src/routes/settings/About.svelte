<script lang="ts">
    import {onMount} from "svelte";
    import * as tauri_path from "@tauri-apps/api/path";
    import {convertFileSrc} from "@tauri-apps/api/core";
    import {type AppDetails, commands} from "../../bindings";

    export let app: AppDetails;

    let icon_path: string;
    onMount(async () => {
        let resource_dir = await tauri_path.resourceDir();
        icon_path = convertFileSrc(`${resource_dir}/icons/128x128.png`);
    });

    function openUrl(url: string) {
        commands.openBrowser(url, true);
    }

</script>

<div class="flex justify-center bg-accent text-gray-400 h-full rounded-xl">
    <div class="flex flex-col items-center justify-center">
        <div class="flex space-x-2 items-center mb-4 p-4 rounded-2xl">
            <img alt="mm" class="h-16 w-16" src="{icon_path}">
            <div>
                <p class="text-2xl font-normal text-primary">Drink Now!</p>
                <p class="text-m text-gray-100 font-extralight">Version {app.version}</p>
            </div>
        </div>
        <div class="flex justify-center text-xs font-normal space-x-4 mb-4">
            <button class="" on:click={() => openUrl("https://drinknow.app?utm_source=app&utm_medium=about")}>Homepage</button>
            <p>|</p>
            <button class="" on:click={() => openUrl("https://drinknow.app/imprint?utm_source=app&utm_medium=about")}>Imprint</button>
            <p>|</p>
            <button class="" on:click={() => openUrl("https://drinknow.app/privacy?utm_source=app&utm_medium=about")}>Privacy</button>
        </div>
        <div class="text-xs">
            <p>Copyright 2024 Rocket Solutions S.L.</p>
        </div>
    </div>
</div>