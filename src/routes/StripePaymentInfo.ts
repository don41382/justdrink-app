import {fetch} from "@tauri-apps/plugin-http";

export namespace StripePaymentInfo {

    interface Response {
        price: number,
        trialDays: string,
        paymentStatus: PaymentStatus
    }

    export enum PaymentStatus {
        READY_TO_CAPTURE = "READY_TO_CAPTURE",
        PAID = "PAID",
        START = "START",
        CANCELED = "CANCELED",
        REQUIRE_INFO = "REQUIRE_INFO"
    }

    export interface Info {
        trialDays: string,
        priceFormatted: string,
        paymentStatus: PaymentStatus
    }

    export async function fetchPaymentInfo(deviceId: string): Promise<Info> {
        const responseRaw = await fetch(`http://drinknow.test:8080/pricing/payment/info/${deviceId}`, {
            method: 'GET'
        })

        if (!responseRaw.ok) {
            throw new Error(`Can't access payment network: ${responseRaw.statusText}`);
        }

        const response = await responseRaw.json() as Response;

        return {
            trialDays: response.trialDays,
            priceFormatted: formatPrice(response.price),
            paymentStatus: response.paymentStatus
        }
    }

    export async function cancelPayment(deviceId: string): Promise<{}> {
        const responseRaw = await fetch(`http://drinknow.test:8080/pricing/payment/cancel/${deviceId}`, {
            method: 'POST',
        });

        if (!responseRaw.ok) {
            throw new Error(`Can't access payment network: ${responseRaw.statusText}`);
        }

        return {}
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