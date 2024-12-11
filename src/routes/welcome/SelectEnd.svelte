<script lang="ts">

    import {commands} from "../../bindings";
    import {onMount} from "svelte";

    let {email = $bindable(), consent = $bindable()}: {
        email: string,
        consent: boolean
    } = $props();

    let emailInput: HTMLInputElement;

    onMount(() => {
        emailInput?.focus();
    })

    function openUrl(url: string) {
        commands.openBrowser(url, false);
    }

</script>

<div class="w-full">
    <div class="mb-4">
        <h2 class="text-lg font-semibold tracking-wide ">Stay up-to-date!</h2>
        <p>Stay in the loop to get update on new features, exercises and tips!</p>
    </div>
    <div class="mb-4">
        <label class="block text-gray-700 text-sm font-normal mb-1" for="email">
            E-Mail
        </label>
        <input bind:this={emailInput} bind:value={email}
               class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
               id="email" placeholder="Your Email" type="text">
    </div>
    <div class="mb-2 flex items-start cursor-pointer">
        <input
                bind:checked={consent}
                class="size-4 mr-2 rounded border-neutral-300 text-neutral-600 focus:ring-neutral-500" id="consent"
                type="checkbox">
        <label class="text-sm text-neutral-600" for="consent">I agree to receive emails from Motion Minute and
            understand I can unsubscribe anytime. See our
            <a class="underline cursor-pointer" onclick={() => openUrl("https://www.motionminute.app/privacy")}>Privacy
                Details</a>
            for details.
        </label>
    </div>
</div>