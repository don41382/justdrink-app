import {
    loadStripe,
    type Stripe,
    type StripeElementsOptionsClientSecret,
    type StripeExpressCheckoutElement, type StripePaymentElement
} from "@stripe/stripe-js";
import {fetch} from '@tauri-apps/plugin-http';
import {info} from "@tauri-apps/plugin-log";
import type {StripeElementsOptionsMode} from "@stripe/stripe-js/dist/stripe-js/elements-group";

interface PaymentResponse {
    clientSecret: string;
    pubKey: string;
}


async function fetchPayment(): Promise<PaymentResponse> {
    const response = await fetch('http://drinknow.test:8080/pricing/drinknow/payment');

    if (!response.ok) {
        throw new Error(`Can't access payment network: ${response.statusText}`);
    }

    return await response.json() as PaymentResponse;
}

export async function fetchAndInitStripe(): Promise<StripePaymentElement> {
    const payment = await fetchPayment()
    const stripe: Stripe | null = await loadStripe(payment.pubKey)
    if (stripe) {
        const options: StripeElementsOptionsClientSecret = {
            clientSecret: payment.clientSecret,
            appearance: {
                disableAnimations: true,
                variables: {
                    colorText: "#8ECAE6",
                    colorBackground: "#001621",
                    colorWarning: "#FB8500",
                    colorDanger: "#FB8500",
                    logoColor: "#FFB703",
                    tabIconSelectedColor: "#FFB703",
                    tabIconColor: "#FFB703",
                    colorPrimary: "#8ECAE6",
                    colorTextSecondary: "#8ECAE6",
                }
            }
        }
        const elements = stripe.elements(options)
        return elements.create("payment", {
            layout: 'accordion'
        })
    } else {
        throw new Error("loadStripe returns null")
    }
}