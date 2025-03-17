<script lang="ts">
    import Navigation from "./Navigation.svelte";
    import type {LicenseData, WelcomeWizardMode} from "../../bindings";
    import {PriceFormatter} from "../PriceFormatter.js";

    let {licenseData, backVisible, welcomeWizardMode, back, next}: {
        licenseData: LicenseData,
        backVisible: boolean,
        welcomeWizardMode: WelcomeWizardMode,
        back: () => void,
        next: () => void
    } = $props()
</script>

<div class="flex-1">
    <div class="flex flex-col w-full h-full">
        {#if licenseData.payment.trial_days_left > 0}
            <h1 class="flex-none text-4xl text-primary text-left mb-2">Try it for {licenseData.payment.trial_days_left}
                {licenseData.payment.total_trail_days !== licenseData.payment.trial_days_left ? "more" : ""}
                days!</h1>
            {#if welcomeWizardMode === "CancelPayment"}
                <p class="text-highlight mb-2">You're payment was cancelled. You can still enjoy your trial.</p>
            {/if}
            <span class="text-secondary/80 font-light">
                We want to ensure this is the perfect app for you. Try it for free, and if you love it, get the
                <span class="text-primary">lifetime license</span> for just <span
                    class="text-primary">{PriceFormatter.format(licenseData.payment.purchase_price)}</span>
            </span>
        {:else}
            <h1 class="flex-none text-4xl text-primary text-left mb-2">Your trial is over</h1>
            <span class="text-secondary/80 font-light">
                I hope you enjoyed Just Drink! - if you like it, you can continue by purchasing the <span
                    class="text-primary">lifetime license</span> for just <span
                    class="text-primary">{PriceFormatter.format(licenseData.payment.purchase_price)}</span>
            </span>
        {/if}
        <div class="flex flex-col flex-1 w-full justify-center items-start mt-8 ml-8">
            <ul class="text-secondary max-w-md space-y-1 list-inside">
                <li><span class="mr-2">⏰</span> Immersive Reminders — Makes you want to drink</li>
                <li><span class="mr-2">💦</span> No workflow disruption — Just drink</li>
                <li><span class="mr-2">🎉</span> {licenseData.payment.total_trail_days}-Day Free Trial — No charge until
                    your trial ends
                </li>
            </ul>
        </div>
    </div>
</div>
<Navigation back={back}
            backVisible={backVisible}
            next={next}
            nextBackground="bg-primary"
            nextDisabled={false}
            nextName={licenseData.payment.trial_days_left > 0 ? 'Start Free Trial' : 'Buy now'}
            nextVisible={true}/>
