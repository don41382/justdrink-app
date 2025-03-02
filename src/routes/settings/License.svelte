<script lang="ts">
    import {
        type AppDetails,
        commands, type LicenseData,
    } from '../../bindings';
    import {info, warn} from "@tauri-apps/plugin-log";
    import {fetch} from '@tauri-apps/plugin-http';
    import {fade} from 'svelte/transition';
    import LicensePayMessage from "./LicensePayMessage.svelte";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {onMount} from "svelte";
    import LoadingSpinner from "../welcome/LoadingSpinner.svelte";

    let {app}: { app: AppDetails } = $props();

    let dataPromise: Promise<LicenseData> = $state(Promise.reject("failed to load license data"))

    onMount(async() => {
        dataPromise = commands.requestLicenseStatus()
    })

    async function purchase() {
        await info("get a license")
        await commands.welcomeOnlyPayment()
        await getCurrentWindow().destroy()
    }

    async function reload() {
        dataPromise = commands.requestLicenseStatus()
    }

    async function cancelPayment(deviceId: string): Promise<void> {
        const url = `${app.url}/app/v1/payment/cancel/${deviceId}`;
        await info(`request: ${url}`)
        const responseRaw = await fetch(url, {
            method: 'POST',
        });

        if (!responseRaw.ok) {
            throw new Error(`Can't access payment network: ${responseRaw.statusText}`);
        }

        return;
    }

    async function cancelPurchase() {
        await info("cancel purchase")
        await cancelPayment(app.device_id).catch((err) => {
            warn(`unable to cancel payment: ${err}, url: ${app.url}, device-id: ${app.device_id}`)
        })
        await getCurrentWindow().destroy()
    }

</script>

<div class="space-y-6 h-full">
    {#await dataPromise}
        <LoadingSpinner fullScreen={false}/>
    {:then data}
        <div class="flex justify-between items-center">
            <h2 class="text-lg font-semibold text-accent">License Status</h2>
            <div class="flex items-center rounded-full px-3 py-1 text-sm bg-gray-200 {data.info.status === 'Invalid'  ? 'text-highlight' : 'text-black'}">
                {#if data.info.message}
                    {data.info.message}
                {/if}
            </div>
        </div>

        <div class="flex-col">
            {#if data.info.status === 'Full' || data.info.status === 'Paid'}
                <LicensePayMessage/>
            {:else}
                <div class="flex flex-col mt-8" transition:fade>
                    {#if data.info.status === "Trial"}
                        {#if data.payment.payment_status === "Start"}
                            <p class="text-gray-700 mb-4">
                                You can try Drink Now! for a few days for free or buy it now.
                            </p>
                            <button class="bg-primary border border-gray-300 text-white rounded-lg px-4 py-2 mx-auto mt-4"
                                    onclick={async () => purchase()}>
                                Buy Now
                            </button>
                        {:else}
                            {#if (data.payment.payment_status === "ReadyToCapture")}
                                <p class="text-gray-700">
                                    Enjoy your trial! You will only be charged after your trial period.
                                </p>
                                <div class="flex w-full justify-center">
                                    <button class="text-gray-600 underline-offset-2 px-4 underline mt-4"
                                            onclick={async () => cancelPurchase()}>
                                        I would like to cancel my trial
                                    </button>
                                </div>
                            {:else if data.payment.payment_status === "RequireInfo" }
                                <p class="text-gray-700">
                                    Enjoy your trial! You still have a few days left to test it out!
                                </p>
                                <button class="bg-primary border border-gray-300 text-white rounded-lg px-4 py-2 mx-auto mt-4"
                                        onclick={async () => purchase()}>
                                    Buy Now
                                </button>
                            {:else if data.payment.payment_status === "Paid"}
                                <LicensePayMessage/>
                            {:else if data.payment.payment_status === "Canceled"}
                                <p class="text-gray-700">
                                    We're sorry to see you leave. We'd love to hear your feedback on how we can improve.
                                    If you change your mind, you can purchase it now!
                                </p>
                                <button class="bg-primary border border-gray-300 text-white rounded-lg px-4 py-2 mx-auto mt-4"
                                        onclick={async () => purchase()}>
                                    Buy Now
                                </button>
                            {/if}
                        {/if}
                    {:else}
                        <p class="text-gray-700 mb-4">
                            Your trial has ended. Please purchase the full version to continue using Drink Now!.
                        </p>
                        <button class="bg-primary border border-gray-300 text-white rounded-lg px-4 py-2 mx-auto"
                                onclick={async () => purchase()}>
                            Buy Now
                        </button>
                    {/if}
                </div>
            {/if}
        </div>
    {:catch err}
        <p class="text-black">I am sorry, something went wrong. Error: {err}</p>
        <button class="bg-accent border border-gray-300 text-white rounded-lg px-4 py-2 mx-auto mt-4"
                onclick={async () => reload()}>
            Retry
        </button>
    {/await}
</div>