<script lang="ts">

    import Navigation from "./Navigation.svelte";
    import {StripePaymentInfo} from "./StripePaymentInfo";

    let {paymentInfo, back, next}: {
        paymentInfo: StripePaymentInfo.Info,
        back: () => void,
        next: () => void
    } = $props();

</script>

<div class="flex-1">
    <div class="w-full">
        <div class="mb-8">
            <h1 class="text-4xl text-primary text-left mb-2">Thank you!</h1>
            <p class="text-secondary/80 font-light mb-2">Thanks for supporting us</p>
        </div>
        <div class="mb-4">
            <div class="p-4 bg-secondary/50 text-white rounded-md">
                {#if paymentInfo.paymentStatus === "PAID"}
                    <p>Your license is active and valid.</p>
                {:else}
                    <p>Enjoy your trail!</p>
                {/if}
            </div>
        </div>
    </div>
</div>

<Navigation back={back}
            backVisible={true}
            next={next}
            nextBackground="bg-primary"
            nextDisabled={false}
            nextName="Finish"/>