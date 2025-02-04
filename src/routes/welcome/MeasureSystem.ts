export enum MeasureSystem {
    Metric = "Metic",
    Imperial = "Imperial"
}

// Companion namespace to extend MeasureSystem with static methods
export namespace MeasureSystem {
    /**
     * Retrieves the country code from a given locale string.
     * @param locale - A locale string (e.g., "en-US").
     * @returns The extracted country code (e.g., "US"), or "UNKNOWN" if not found.
     */
    function getCountryCode(locale: string | undefined): string {
        if (!locale) return "UNKNOWN";

        const parts = locale.split("-");
        return parts.length > 1 ? parts.pop()?.toUpperCase() || "UNKNOWN" : "UNKNOWN";
    }

    /**
     * Determines the measurement system based on the user's locale.
     * @returns The appropriate MeasureSystem (Metric or Imperial).
     */
    export function getMeasureSystem(): MeasureSystem {
        try {
            // Get the user's locale
            const locale = new Intl.DateTimeFormat().resolvedOptions().locale; // Example: "en-US"

            // Extract country code safely
            const countryCode = getCountryCode(locale);

            // List of countries that use the imperial system
            const imperialCountries: Set<string> = new Set(["US", "LR", "MM"]); // USA, Liberia, Myanmar

            // Return the measurement system based on country code
            return imperialCountries.has(countryCode) ? MeasureSystem.Imperial : MeasureSystem.Metric;
        } catch (error) {
            console.error("Error detecting measurement system:", error);
            return MeasureSystem.Metric;
        }
    }
}
