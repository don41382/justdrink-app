<script lang="ts">
    import {info, warn} from "@tauri-apps/plugin-log";
    import type {StripeExpressCheckoutElement, StripePaymentElement} from '@stripe/stripe-js'
    import {onMount} from "svelte";
    import {fetchAndInitStripe} from "./StripePayment";
    import type {
        StripeExpressCheckoutElementReadyEvent
    } from "@stripe/stripe-js/dist/stripe-js/elements/express-checkout";

    let {paymentElement}: { paymentElement: StripePaymentElement | undefined } = $props();


    let expressCheckoutDiv: HTMLDivElement;

    onMount(async () => {
        const paymentElement = await fetchAndInitStripe().catch((err) => {
            warn(`error: ${err}`)
        });
        if (paymentElement) {
            // paymentElement.once("ready", async (event: StripeExpressCheckoutElementReadyEvent) => {
            //     await info(`payment-options: ${event.availablePaymentMethods}`);
            // })
            // paymentElement.once('loaderror', (err) => {
            //     warn(`error: ${err.error}`)
            // })
            await info("mount element")
            paymentElement.mount("#express-checkout-element")
            await info("element mounted")
        } else {
            expressCheckoutDiv.innerHTML = "Sorry, we could not retrieve the payment information!"
        }
    })
</script>

<div class="flex flex-col w-full h-full">
    <h1 class="text-4xl text-primary text-left mb-2">Try 5 Days, Pay €2.99 Once</h1>
    <p class="text-secondary/80">
        Don't worry, you can cancel the trail within the 5 days!
    </p>
    <div class="mt-4 mb-8">
        <form id="payment-form">
            <div bind:this={expressCheckoutDiv} id="express-checkout-element">
                <!-- Elements will create form elements here -->
            </div>
        </form>
    </div>
    <div id="messages" role="alert" style="display: none;"></div>
</div>