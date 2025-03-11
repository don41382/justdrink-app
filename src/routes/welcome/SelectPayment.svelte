<script lang="ts">
    import {commands, type LicenseData, type WelcomeWizardMode} from "../../bindings";
    import {onDestroy, onMount} from "svelte";
    import Navigation from "./Navigation.svelte";
    import {debug, info, warn} from "@tauri-apps/plugin-log"
    import {getCurrentWindow} from "@tauri-apps/api/window";

    let {licenseData = $bindable(), backendUrl, email, deviceId, welcomeWizardMode, next, back}: {
        licenseData: LicenseData,
        backendUrl: string,
        email: string | null,
        deviceId: string,
        welcomeWizardMode: WelcomeWizardMode,
        next: () => void,
        back: () => void
    } = $props();

    const MAX_RETRIES = 300

    let paymentCheckerTimer: number = 0

    let loading: boolean = $state(true)
    let retry: boolean = $state(false)
    let message: string = $state("Please wait")
    let error: string | undefined = $state(undefined)
    let retriesCount: number = $state(MAX_RETRIES)

    async function startPaymentChecker() {
        loading = true
        retry = false
        error = undefined
        message = "Please wait"
        retriesCount = MAX_RETRIES
        await paymentChecker()
    }

    async function cancel(reason: string, errorMessage: string | undefined = undefined) {
        loading = false
        retry = true
        message = reason
        error = errorMessage
    }

    async function paymentChecker() {
        await commands.requestLicenseStatus().then(async (license) => {
            licenseData = license
            if (license.info.status === "Paid") {
                await debug("selectPayment: license already payed")
                await finishNext()
            }
            switch (license.payment.payment_status) {
                case "GoToCheckout":
                    await debug("selectPayment: waiting for checkout")
                    if (retriesCount == MAX_RETRIES) {
                        await debug("selectPayment: open payment, first check")
                        message = "You will be redirected to our payment site."
                        error = "Waiting for payment ..."
                        setTimeout(async () => {
                            await commands.openPayment()
                        }, 2000)
                    }
                    retriesCount -= 1
                    if (retriesCount >= 0) {
                        paymentCheckerTimer = setTimeout(paymentChecker, 1000);
                    } else {
                        loading = false
                        retry = true
                        message = "Payment timed out"
                    }
                    break;
                case "ReadyToCapture":
                    await debug("selectPayment: done")
                    await finishNext()
                    break;
                case "Paid":
                    await debug("selectPayment: payment already done")
                    await finishNext()
                    break;
                case "Canceled":
                    await debug("selectPayment: payment canceled")
                    await cancel("The payment was canceled, please retry")
                    break;
                case "Error":
                    await debug("selectPayment:error while payment")
                    await cancel("There was an error with the payment. Please retry. If the error persists, please write us: info@rocket-solutions.de")
                    break;

            }
        }).catch(async (err) => {
            await cancel(`We have problem reaching the license server.`, err)
        })
    }


    onMount(async () => {
        await info(`selectPayment: opening payment in default browser, license status: ${licenseData.payment.payment_status}`)
        await startPaymentChecker()
    })

    async function finishNext() {
        loading = false;
        await getCurrentWindow().setFocus()
        next()
    }

    onDestroy(async () => {
        await info("selectPayment: close")
        await getCurrentWindow().setFocus()
        clearTimeout(paymentCheckerTimer)
    })

</script>

<div class="flex-1">
    <div class="absolute inset-0 flex flex-col items-center justify-center bg-black bg-opacity-50 z-40 rounded-2xl px-24">
        {#if loading}
            <svg aria-hidden="true" class="w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-primary"
                 fill="none" viewBox="0 0 100 101" xmlns="http://www.w3.org/2000/svg">
                <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                      fill="currentColor"/>
                <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                      fill="currentFill"/>
            </svg>
        {/if}
        <p class="mt-4 text-white text-center">{message}</p>
        {#if error}
            <p class="text-highlight">{error}</p>
        {/if}
        {#if retry}
            <button class="text-secondary/80 py-2 rounded-md mt-1" onclick={startPaymentChecker}>
                Retry
            </button>
        {/if}
    </div>
</div>
<Navigation back={back}
            backVisible={false}
            next={() => {}}
            nextBackground="bg-primary"
            nextDisabled={true}
            nextName="Next"
            nextVisible={false}/>
