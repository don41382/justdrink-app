<script lang="ts">

    import Navigation from "./Navigation.svelte";
    import {commands, type LicenseData} from "../../bindings";
    import {onMount} from "svelte";
    import {info} from "@tauri-apps/plugin-log";

    let {licenseData, back, backVisible}: {
        licenseData: LicenseData,
        backVisible: boolean,
        back: () => void,
    } = $props();

    onMount(async () => {
        await info(`thank you: ${licenseData.info.status}, ${licenseData.payment.payment_status}`)
    })

    async function close() {
        await commands.welcomeClose("ThankYou")
    }

    function thankYouMessage(): string {
        if (licenseData.info.status == "Full") {
            return "Enjoy Just Drink!"
        } else
            switch (licenseData.payment.payment_status) {
                case "GoToCheckout":
                    return "Enjoy your trail."
                case "ReadyToCapture":
                    return "Thanks for providing the payment details."
                case "Paid":
                    return "Your app is paid. Thanks for supporting us."
                case "Canceled":
                    return "Please restart the payment process. Your payment seems to be canceled."
                case "Error":
                    return "Something went wrong with your payment."

            }
    }

</script>

<div class="flex-1">
    <div class="w-full">
        <div class="mb-4">
            <h1 class="text-4xl text-primary text-left mb-2">Thank you!</h1>
            <p class="text-secondary/80 font-light mb-2">{thankYouMessage()}</p>
        </div>
        <div>
            <div class="p-4 bg-secondary/20 text-white rounded-md">
                {#if licenseData.info.status === "Paid" || licenseData.info.status === "Full" || licenseData.payment.payment_status === "Paid"}
                    <p>Your license is active and valid.</p>
                {:else}
                    <p>Enjoy your trial for {licenseData.payment.trial_days_left} days! You can cancel your trial via
                        Settings -> License.</p>
                {/if}
            </div>
        </div>
    </div>
</div>

<Navigation back={back}
            backVisible={backVisible}
            next={close}
            nextBackground="bg-primary"
            nextDisabled={false}
            nextName="Finish"
            nextVisible={true}/>