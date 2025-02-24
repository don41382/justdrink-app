import {fetch} from "@tauri-apps/plugin-http";

export namespace StripePaymentInfo {

    interface Response {
        price: number,
        trialDays: string
    }

    export interface Info {
        trialDays: string,
        priceFormatted: string
    }

    export async function fetchPaymentInfo(): Promise<Info> {
        const responseRaw = await fetch(`http://drinknow.test:8080/pricing/payment/info`, {
            method: 'GET'
        })

        if (!responseRaw.ok) {
            throw new Error(`Can't access payment network: ${responseRaw.statusText}`);
        }

        const response = await responseRaw.json() as Response;

        return {
            trialDays: response.trialDays,
            priceFormatted: formatPrice(response.price)
        }
    }

    export function formatPrice(price: number): string {
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