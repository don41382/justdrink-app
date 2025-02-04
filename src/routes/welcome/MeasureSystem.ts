export enum MeasureSystem {
    Metric,
    Imperial
}

function getCountryCode(locale: string | undefined): string {
    if (!locale) return "UNKNOWN"; // Handle missing or invalid values

    const parts = locale.split("-");
    return parts.length > 1 ? parts.pop()?.toUpperCase() || "UNKNOWN" : "UNKNOWN";
}

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
        return MeasureSystem.Metric
    }
}

