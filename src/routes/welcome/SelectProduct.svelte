<script lang="ts">
    import Navigation from "./Navigation.svelte";
    import type {LicenseData} from "../../bindings";
    import {PriceFormatter} from "../PriceFormatter.js";

    let {licenseData, backVisible, back, next}: {
        licenseData: LicenseData,
        backVisible: boolean,
        back: () => void,
        next: () => void
    } = $props()

</script>

<div class="flex-1">
    <div class="flex flex-col w-full h-full">
        <h1 class="flex-none text-4xl text-primary text-left mb-2">Try it for {licenseData.payment.total_trail_days} days!</h1>
        <span class="text-secondary/80 font-light">
                We want to ensure this is the perfect app for you. Try it for free, and if you love it, get the
                <span class="text-primary">lifetime license</span> for just <span
                class="text-primary">{PriceFormatter.format(licenseData.payment.purchase_price)}</span>.
            </span>
        <div class="flex flex-col flex-1 w-full justify-center items-start mt-8 ml-8">
            <ul class="text-secondary max-w-md space-y-1 list-inside">
                <li><span class="mr-2">⏰</span> Immersive Reminders — Makes you want to drink</li>
                <li><span class="mr-2">💦</span> No workflow disruption — Just drink</li>
                <li><span class="mr-2">🎉</span> 3-Day Free Trial — No charge until your trial ends</li>
            </ul>
        </div>
    </div>
</div>
<Navigation back={back}
            backVisible={backVisible}
            next={next}
            nextBackground="bg-primary"
            nextDisabled={false}
            nextName="Start Free Trial"/>
