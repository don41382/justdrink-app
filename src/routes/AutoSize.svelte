<script lang="ts">
    import {onMount, onDestroy, tick} from 'svelte';
    import {getCurrentWindow, PhysicalSize} from '@tauri-apps/api/window';
    import {type} from "@tauri-apps/plugin-os"
    import type { Snippet } from 'svelte';
    import {debug} from "@tauri-apps/plugin-log";

    interface Props {
        ready: boolean;
        children: Snippet;
        [key: string]: unknown;
    }

    let {ready = true, children, ...rest}: Props = $props();

    let container: HTMLDivElement | null = $state(null);

    async function resizeWindow() {
        const currentWindow = getCurrentWindow();
        await debug("resizeWindow called");

        if (container && ready) {
            await tick();
            let rect = container.getBoundingClientRect()
            const factor = window.devicePixelRatio;
            const width: number = Math.ceil(rect.width * factor);
            const height: number = Math.ceil(rect.height * factor);

            let topPadding = await currentWindow.isDecorated() && type() === 'macos' ? 55 : 0
            let size = new PhysicalSize(width, height + topPadding);

            await debug(`resizing now to ${width}x${height + topPadding}`)

            await currentWindow.setSize(size);
            await getCurrentWindow().center();
            await getCurrentWindow().show();
            await getCurrentWindow().setFocus();
        }
    }

    let observer: ResizeObserver;

    onMount(async () => {
        observer = new ResizeObserver(async () => {
            await resizeWindow();
        });
        if (container) observer.observe(container);
    });

    onDestroy(() => {
        if (observer) observer.disconnect();
    });
</script>

<div {...rest} bind:this={container}>
    {@render children?.()}
</div>