<script lang="ts">
    import AutoSize from "../AutoSize.svelte";
    import Error from "./Error.svelte";
    import {commands} from "../../bindings";
    import LoadingSpinner from "./LoadingSpinner.svelte";
    import Wizard from "./Wizard.svelte";
    import type {WelcomeStep} from "./WelcomeStep";
    import Xmark from "../../icons/Xmark.svelte";

    let {data} = $props();

    let licenseDataPromise = $state(data.licenseData)
    let currentStep: WelcomeStep | undefined = $state(undefined)
    let isClosing: boolean = $state(false)

    async function close() {
        isClosing = true
        await commands.welcomeClose(currentStep ?? "Start").catch(() => {
            isClosing = false;
        })
    }

    async function reload() {
        licenseDataPromise = commands.requestLicenseStatus()
    }

</script>

<AutoSize
        class="flex flex-col bg-accent w-[650px] min-h-[500px] px-12 cursor-default rounded-2xl"
        ready={true}>
    <div class="flex pt-8 mb-4 justify-between items-center">
        <div class="flex items-center">
            <img alt="Drink Now!" class="size-12" src="{data.iconPath}">
            <p class="text-xl ml-2 text-primary">Drink Now!</p>
        </div>
        <button class="flex flex-col items-center justify-center cursor-pointer rounded-full hover:bg-gray-600 text-secondary/20 hover:text-white p-1 size-8 z-50"
                onclick={async () => { await close() }}>
            <Xmark/>
        </button>
    </div>

    {#await licenseDataPromise}
        <LoadingSpinner fullScreen={true}/>
    {:then licenseData}
        <Wizard currentStep={currentStep} licenseDataInitial={licenseData} welcomeMode={data.welcomeMode}
                images={data.images}
                settings={data.settings}/>
    {:catch error}
        <Error error={error} reload={reload}/>
    {/await}

    {#if isClosing}
        <LoadingSpinner fullScreen={true}/>
    {/if}
</AutoSize>