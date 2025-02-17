<script lang="ts">
    import type {DrinkAudio} from "./+page";
    import type {DrinkCharacter, SipSize} from "../../bindings";
    import {info} from "@tauri-apps/plugin-log";

    let {drinkAudio, selectedDrinkCharacter, lastPlay}: {
        drinkAudio: DrinkAudio,
        selectedDrinkCharacter: DrinkCharacter,
        lastPlay: () => void
    } = $props();

    let sparkling: HTMLAudioElement;
    let channel1: HTMLAudioElement;  // Tracks the currently playing audio element
    let channel2: HTMLAudioElement;  // Tracks the next audio element to use
    let watchChannel: HTMLAudioElement;

    let sparklingTimer: number | undefined

    let sipCount = 3;
    let beforeSipTimeMs = 3000
    let sips: string[] = $derived(drinkAudio.personas[selectedDrinkCharacter].sips);
    let ahh: string[] = $derived(drinkAudio.personas[selectedDrinkCharacter].ahh);

    let selectedStart: string[] = [];
    let audioQueue: string[] = [];

    function getRandomSubset<T>(array: T[], count: number): T[] {
        let shuffled = [...array].sort(() => Math.random() - 0.5);
        return shuffled.slice(0, count);
    }

    export async function play() {
        cleanup()
        selectedStart = getRandomSubset(sips, Math.min(sipCount + 1, sips.length));
        let selectedEnd = getRandomSubset(ahh, 1);
        audioQueue = [...selectedStart, ...selectedEnd];

        sparkling.volume = 0.8
        channel1.volume = 0.6
        channel2.volume = 0.6

        sparkling.src = getRandomSubset(drinkAudio.sparkling, 1).shift()!;
        sparkling.currentTime = 0
        await sparkling.play()

        sparklingTimer = setTimeout(async () => {
            await info("timer finished")
            if (audioQueue.length > 0) {
                channel1.src = audioQueue.shift()!
                channel1.currentTime = 0
                watchChannel = channel1
                await channel1.play()
            }
        }, beforeSipTimeMs)
    }

    function playNext(channel: HTMLAudioElement) {
        if (!sparklingTimer) {
            return;
        }
        if (audioQueue.length === 0) {
            return;
        }
        if (audioQueue.length === 1) {
            lastPlay();
        }
        channel.src = audioQueue.shift()!
        info(`play: ${channel.src}`)
        channel.load()
        channel.play()
        watchChannel = channel;
    }

    function checkTimeUpdate(event: Event) {
        let playingAudio = event.target as HTMLAudioElement;
        if (playingAudio === watchChannel) {
            if ((playingAudio.duration > 0.8 && playingAudio.paused) || (playingAudio.duration - playingAudio.currentTime <= 0.2)) {
                const freeChannel = playingAudio == channel1 ? channel2 : channel1
                playNext(freeChannel);
            }
        }
    }

    function cleanup() {
        clearTimeout(sparklingTimer)
        sparklingTimer = undefined

        if (sparkling) {
            sparkling.pause()
        }
        if (channel1)  {
            channel1.pause()
        }
        if (channel2) {
            channel2.pause()
        }
    }

</script>
<audio bind:this={sparkling}></audio>
<audio bind:this={channel1} ontimeupdate={checkTimeUpdate}></audio>
<audio bind:this={channel2} ontimeupdate={checkTimeUpdate}></audio>