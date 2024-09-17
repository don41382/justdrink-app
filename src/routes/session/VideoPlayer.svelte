<script lang="ts">
    export let filename: string;
    let className = '';
    export { className as class };

    let videoElement: HTMLVideoElement;
    let videoReady = false;

    function setVideoReady() {
        if (videoElement.readyState === 4) {
            videoReady = true;
        }
    }

    $: ready = videoReady;
</script>

<style>
    .video-ready {
        opacity: 1;
        transition: opacity 4s;
    }

    .video-not-ready {
        opacity: 0;
    }
</style>

<video
        bind:this={videoElement}
        on:canplay={setVideoReady}
        class="{className} {ready ? 'video-ready' : 'video-not-ready'}"
        autoplay loop muted playsinline preload="metadata">
    <source
            src="/videos/{filename}.mov"
            type='video/mp4; codecs="hvc1"'>
    <source
            src="/videos/{filename}.webm"
            type="video/webm">
    Your browser does not support the video tag.
</video>