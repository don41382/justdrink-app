<script lang="ts">
    import {
        type AppDetails,
        commands,
    } from '../../bindings';
    import {info} from "@tauri-apps/plugin-log";
    import {fade} from 'svelte/transition';
    import {StripePaymentInfo} from "../StripePaymentInfo";
    import {onMount} from "svelte";
    import LicensePayMessage from "./LicensePayMessage.svelte";

    let {app}: { app: AppDetails } = $props();

    let paymentInfo: Promise<StripePaymentInfo.Info> = $state(Promise.reject("waiting for payment info"))

    onMount(async () => {
        await load();
    })

    async function load() {
        paymentInfo = StripePaymentInfo.fetchPaymentInfo(app.device_id)
    }

    async function purchase() {
        await info("get a license")
        await commands.welcomeOnlyPayment()
    }

    async function cancelPurchase() {
        await info("cancel purchase")
        await StripePaymentInfo.cancelPayment(app.device_id)
        await load();
    }

</script>

<div class="space-y-6">
    <div class="flex justify-between items-center">
        <h2 class="text-lg font-semibold text-accent">License Status</h2>
        <div class="flex items-center rounded-full px-3 py-1 text-sm bg-gray-200 {app.license_info.status === 'Invalid'  ? 'text-highlight' : 'text-black'}">
            {#if app.license_info.message}
                {app.license_info.message}
            {/if}
        </div>
    </div>

    <div class="flex-col">
        {#if app.license_info.status === 'Full' || app.license_info.status === 'Paid'}
            <LicensePayMessage/>
        {:else}
            {#await paymentInfo}
                <p class="text-gray-600 text-sm text-center">Retrieving payment info ...</p>
            {:then info}
                <div class="flex flex-col mt-8" transition:fade>
                    {#if app.license_info.status === "Trial"}
                        {#if info.paymentStatus === "START"}
                            <p class="text-gray-700 mb-4">
                                You can try Drink Now! for a few days for free or buy it now.
                            </p>
                            <button class="bg-primary border border-gray-300 text-white rounded-lg px-4 py-2 mx-auto mt-4"
                                    onclick={async () => purchase()}>
                                Buy Now
                            </button>
                        {:else}
                            {#if (info.paymentStatus === "READY_TO_CAPTURE")}
                                <p class="text-gray-700">
                                    Enjoy your trial! You will only be charged after your trial period.
                                </p>
                                <div class="flex w-full justify-center">
                                    <button class="text-gray-600 underline-offset-2 px-4 underline mt-4"
                                            onclick={async () => cancelPurchase()}>
                                        I would like to cancel my trial
                                    </button>
                                </div>
                            {:else if info.paymentStatus === "REQUIRE_INFO" }
                                <p class="text-gray-700">
                                    Enjoy your trial! You still have a few days left to test it out!
                                </p>
                                <button class="bg-primary border border-gray-300 text-white rounded-lg px-4 py-2 mx-auto mt-4"
                                        onclick={async () => purchase()}>
                                    Buy Now
                                </button>
                            {:else if info.paymentStatus === "PAID"}
                                <LicensePayMessage/>
                            {:else if info.paymentStatus === "CANCELED"}
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
            {:catch err}
                <p class="text-black">I am sorry, something went wrong. Error: {err}</p>
                <button class="bg-accent border border-gray-300 text-white rounded-lg px-4 py-2 mx-auto mt-4"
                        onclick={async () => load()}>
                    Retry
                </button>
            {/await}
        {/if}
    </div>
</div>