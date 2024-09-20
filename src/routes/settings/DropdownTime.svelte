<script lang="ts">
    export let selectedItem: number | null = null;

    let isOpen: boolean = false;
    
    function toggleDropdown(): void {
        isOpen = !isOpen;
    }

    function selectItem(item: number): void {
        selectedItem = item;
        isOpen = false;
    }
</script>

<button class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
        id="dropdownDefaultButton"
        on:click={toggleDropdown} type="button">
    {selectedItem ? selectedItem : "Dropdown button"}
    <svg aria-hidden="true" class="w-2.5 h-2.5 ms-3" fill="none" viewBox="0 0 10 6" xmlns="http://www.w3.org/2000/svg">
        <path d="m1 1 4 4 4-4" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"/>
    </svg>
</button>

<!-- Dropdown menu -->
{#if isOpen}
    <div id="dropdown" class="z-10 bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700">
        <ul class="py-2 text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdownDefaultButton">
            <li>
                <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                   on:click={() => selectItem(10)}>10 minutes</a>
            </li>
            <li>
                <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                   on:click={() => selectItem(30)}>30 minutes</a>
            </li>
            <li>
                <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                   on:click={() => selectItem(60)}>1 hour</a>
            </li>
            <li>
                <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                   on:click={() => selectItem(180)}>3 hours</a>
            </li>
        </ul>
    </div>
{/if}

<style>
    #dropdownDefaultButton {
        cursor: pointer;
    }
</style>