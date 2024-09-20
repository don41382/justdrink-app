<script lang="ts">

    import {commands} from '../../bindings';

    let backgroundVideo: HTMLVideoElement;
    let backgroundVideoReady = false;

    let next_break_duration_minutes: string = "180"

    function setBackgroundVideoReady() {
        if (backgroundVideo.readyState === 4) {
            backgroundVideoReady = true
        }
    }

    function startSession() {
        commands.startFirstSession(parseInt(next_break_duration_minutes));
    }

    // allows no context menu
    document.addEventListener('contextmenu', event => event.preventDefault());
</script>


<div class="{backgroundVideoReady ? 'video-background-ready' : 'video-not-ready'} h-screen flex flex-col items-center justify-center cursor-default">
    <div class="relative z-10 flex flex-col items-center p-8">
        <h1 class="text-4xl mb-4 accent-mm-blue">Welcome to</h1>
        <h2 class="text-6xl text-mm-orange mb-8">Motion Minutes</h2>
        <p class="px-16 text-xl text-center italic mb-16 block">
            Stay active and energized throughout your day with gentle reminders to stretch and move.
        </p>
        <div class="flex items-center space-x-4 mb-8">
            <p class="text-xl">Start my session every</p>
            <select
                    bind:value={next_break_duration_minutes}
                    class="p-2 bg-transparent bg-mm-pink-100 rounded-l shadow-sm text-right text-black w-24">
                <option class="p-2" value=10>10 min</option>
                <option class="p-2" value=30>30 min</option>
                <option class="p-2" value=60>1 hour</option>
                <option class="p-2" value=120>2 hours</option>
                <option class="p-2" value=180>3 hours</option>
            </select>
        </div>
        <div>
            <button class="bg-mm-orange-100 hover:bg-mm-orange hover:text-white text-mm-blue-500 font-medium py-2 px-4 rounded-xl text-xl cursor-pointer"
                    on:click={startSession}>
                Start your first session
            </button>
        </div>
    </div>
    <video
            autoplay
            bind:this={backgroundVideo}
            class="absolute w-full h-full object-cover rounded-3xl p-2"
            loop muted on:canplay={setBackgroundVideoReady} playsinline
            preload="auto">
        <source src="/videos/bg-h264.mov" type="video/mp4">
        Your browser does not support the video tag.
    </video>
</div>

<style>
    .video-background-ready {
        opacity: 1.0;
        transition: opacity 1s;
    }

    .video-not-ready {
        opacity: 0;
    }
</style>