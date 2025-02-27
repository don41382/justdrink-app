export namespace PriceFormatter {
    export function format(price: number): string {
        const locale = navigator.language || 'en-US'

        // Create a NumberFormat instance with desired options
        const formatter = new Intl.NumberFormat(locale, {
            style: 'currency',
            currency: 'eur',
            minimumFractionDigits: 2,
            maximumFractionDigits: 2,
        })

        return formatter.format(price)
    }
}