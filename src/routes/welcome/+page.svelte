<script lang="ts">
    import AutoSize from "../AutoSize.svelte";
    import Error from "./Error.svelte";
    import Icon from "@iconify/svelte";
    import {commands} from "../../bindings";
    import LoadingSpinner from "./LoadingSpinner.svelte";
    import Wizard from "./Wizard.svelte";

    let {data} = $props();

    let licenseDataPromise = $state(data.licenseData)

    async function close() {
        // TODO: add step
        await commands.welcomeClose("Close")
    }

    async function reload() {
        licenseDataPromise = commands.requestLicenseStatus()
    }

</script>

<AutoSize
        class="flex flex-col bg-accent w-[650px] min-h-[450px] px-12 cursor-default rounded-2xl"
        ready={true}>
    <div class="flex pt-8 mb-4 justify-between items-center">
        <div class="flex items-center">
            <img alt="Drink Now!" class="size-12" src="{data.iconPath}">
            <p class="text-xl ml-2 text-primary">Drink Now!</p>
        </div>
        <button class="flex flex-col items-center justify-center cursor-pointer rounded-full hover:bg-gray-600 text-secondary/20 hover:text-white p-1"
                onclick={async () => { await close() }}>
            <Icon class="size-6" icon="iconoir:xmark"/>
        </button>
    </div>

    {#await licenseDataPromise}
        <LoadingSpinner/>
    {:then licenseData}
        <Wizard licenseData={licenseData} welcomeMode={data.welcomeMode} images={data.images} settings={data.settings}/>
    {:catch error}
        <Error error={error} reload={reload}/>
    {/await}
</AutoSize>