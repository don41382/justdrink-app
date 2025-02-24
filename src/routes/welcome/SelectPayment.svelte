<script lang="ts">
    import type {StripePaymentElement} from '@stripe/stripe-js'
    import {commands, type WelcomeWizardMode} from "../../bindings";
    import {onMount} from "svelte";
    import {fetchAndInitStripe, Status, type StripeSetup} from "./StripePayment";
    import type {Action} from "svelte/action";
    import Navigation from "./Navigation.svelte";
    import {info, warn} from "@tauri-apps/plugin-log"
    import {StripePaymentInfo} from "./StripePaymentInfo";
    import LoadingSpinner from "./LoadingSpinner.svelte";

    let {paymentInfo, email, deviceId, welcomeWizardMode, back}: {
        paymentInfo: StripePaymentInfo.Info,
        email: string | null,
        deviceId: string,
        welcomeWizardMode: WelcomeWizardMode,
        back: () => void
    } = $props();

    let stripeSetup: Promise<StripeSetup> = $state(Promise.reject("not yet executed"))

    let loading: boolean = $state(true)
    let nextName: string = $state("Next")

    onMount(async () => {
        await load();
    })

    const mountPayment: Action<HTMLElement, StripePaymentElement> = (node: HTMLDivElement, paymentElement: StripePaymentElement) => {
        paymentElement.mount(node);
        $effect(() => {
            return () => {
                paymentElement.unmount()
            }
        })
    }

    async function load() {
        stripeSetup = fetchAndInitStripe(email, deviceId).then(async (setup) => {
            loading = false
            return setup
        }).then((res) => {
            switch (res.paymentResult.status) {
                case Status.Succeeded:
                    nextName = "Finish"
                    break;

                case Status.RequiresCapture:
                    nextName = "Done"
                    break;

            }
            return res
        }).catch((err) => {
            loading = false
            warn(`unable to access payment backend: ${err}`)
            throw err
        })
    }

    async function nextOrPayNow() {
        const setup = await stripeSetup;
        switch (setup.paymentResult.status) {
            case Status.Succeeded:
            case Status.RequiresCapture:
                await info("next")
                await commands.welcomeClose("PaymentDone")
                break;
            case Status.RequiresPaymentMethod:
                await info("payNow")
                loading = true
                await setup.stripe.confirmPayment({
                    elements: setup.paymentResult.elements,
                    confirmParams: {
                        return_url: `http://localhost:1420/welcome?mode=${"OnlyPayment" as WelcomeWizardMode}`,
                    }
                }).then(() => {
                    loading = false
                    nextName = "Finish"
                }).catch((err) => {
                    warn(`unable to load payment: ${err}`)
                    loading = false
                    throw err
                })
        }
    }
</script>

<div class="flex-1">
    <div class="flex flex-col w-full h-full">
        <h1 class="text-4xl text-primary text-left mb-2">Try {paymentInfo.trialDays} Days Free,
            Pay {paymentInfo.priceFormatted} Once</h1>
        <p class="text-secondary/80 font-light">
            No charge today — only pay {paymentInfo.priceFormatted} if you love the results. Cancel anytime before the
            trial ends if it’s not for you.
        </p>
        <div class="mt-4 mb-4">
            <form id="payment-form">
                {#await stripeSetup}
                    <p class="text-gray-600">Please wait, we are loading the payment details ...</p>
                {:then setup}
                    {#if setup.paymentResult.status === Status.Succeeded}
                        <div class="p-4 bg-secondary/50 text-white rounded-md">
                            <p>Thank you for purchasing Drink Now!</p>
                        </div>
                    {:else if setup.paymentResult.status === Status.RequiresCapture}
                        <div class="p-4 bg-secondary/50 text-white rounded-md">
                            <p>Thank you for trying Drink Now!</p>
                        </div>
                    {:else if setup.paymentResult.status === Status.RequiresPaymentMethod}
                        <div use:mountPayment={setup.paymentResult.paymentElement} class="my-4">
                            <!-- Stripe Payment Element will be mounted here -->
                        </div>
                    {:else}
                        <p class="text-highlight">I am sorry, something went wrong. Please contact our support:
                            info@rocket-solutions.de</p>
                    {/if}
                {:catch error}
                    <p class="text-highlight mt-4">We are unable to retrieve the payment options. Please try again.</p>
                    <p class="text-highlight/50 text-sm">Error reason: "{error}"</p>
                    <button class="bg-primary hover:bg-primary/50 text-black py-2 rounded-md px-8 ml-auto mt-4"
                            onclick={load}>Reload
                    </button>
                {/await}
            </form>
        </div>
        <div id="messages" role="alert" style="display: none;"></div>
    </div>

    {#if loading}
        <LoadingSpinner/>
    {/if}
</div>
<Navigation back={back}
            backVisible={welcomeWizardMode !== "OnlyPayment"}
            next={nextOrPayNow}
            nextBackground="bg-primary"
            nextDisabled={loading}
            nextName={nextName}/>
