<script lang="ts">
    import type {StripePaymentElement} from '@stripe/stripe-js'
    import {commands, type WelcomeWizardMode} from "../../bindings";
    import {onMount} from "svelte";
    import {fetchAndInitStripe, Status, type StripeSetup} from "./StripePayment";
    import type {Action} from "svelte/action";
    import Navigation from "./Navigation.svelte";
    import {info, warn} from "@tauri-apps/plugin-log"
    import {StripePaymentInfo} from "./StripePaymentInfo";

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
                case Status.RequirePaymentMethod:
                    nextName = "Start trial now!"
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
                await info("next")
                await commands.welcomeClose()
                break;
            case Status.RequirePaymentMethod:
                await info("payNow")
                loading = true
                await setup.stripe.confirmSetup({
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
                            <p>Thank you - you are all set. Just wait for your first reminder.</p>
                        </div>
                    {:else if setup.paymentResult.status === Status.RequirePaymentMethod}
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
        <div class="absolute inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50">
            <svg aria-hidden="true" class="w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-primary"
                 viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                      fill="currentColor"/>
                <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                      fill="currentFill"/>
            </svg>
        </div>
    {/if}
</div>
<Navigation back={back}
            backVisible={welcomeWizardMode !== "OnlyPayment"}
            next={nextOrPayNow}
            nextBackground="bg-primary"
            nextDisabled={loading}
            nextName={nextName}/>
