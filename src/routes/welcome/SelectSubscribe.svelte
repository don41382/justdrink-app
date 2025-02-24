<script lang="ts">

    import {commands} from "../../bindings";
    import {onMount} from "svelte";
    import {info} from "@tauri-apps/plugin-log";
    import Navigation from "./Navigation.svelte";

    let {email = $bindable(), consent = $bindable(), back, next}: {
        email: string | null,
        consent: boolean,
        back: () => void,
        next: () => void
    } = $props();

    let emailInput: HTMLInputElement;

    onMount(() => {
        emailInput?.focus();
    })

    function openUrl(url: string) {
        commands.openBrowser(url, false);
    }

    function handleEnter(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            next()
        }
    }

</script>

<div class="flex-1">
    <div class="w-full">
        <div class="mb-8">
            <h1 class="text-4xl text-primary text-left mb-2">Stay up-to-date!</h1>
            <p class="text-secondary/80 font-light mb-2">Stay in the loop to get update on new exercises, features and tips!</p>
        </div>
        <div class="mb-4">
            <label class="block text-secondary/80 text-sm font-medium mb-1" for="email">
                Your E-Mail
            </label>
            <input bind:this={emailInput} bind:value={email}
                   class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                   id="email"
                   onkeydown={handleEnter} placeholder="Your Email" type="text">
        </div>
        <div class="mb-2 flex items-start cursor-pointer">
            <input
                    bind:checked={consent}
                    class="size-4 mr-2 rounded border-neutral-300 focus:ring-neutral-500"
                    id="consent"
                    type="checkbox">
            <label class="text-sm text-secondary/50" for="consent">I agree to receive emails from Drink Now! and
                understand I can unsubscribe anytime. See our
                <button
                        class="underline cursor-pointer"
                        onclick={() => openUrl("https://drinknow.app/privacy?utm_source=app&utm_medium=consent")}>
                    Privacy
                    Details
                </button>
                for details.
            </label>
        </div>
    </div>
</div>

<Navigation back={back}
            backVisible={true}
            next={next}
            nextBackground="bg-primary"
            nextDisabled={false}
            nextName="Next"/>