import {
    loadStripe,
    type Stripe, type StripeElements,
    type StripeElementsOptionsClientSecret,
    type StripeExpressCheckoutElement, type StripePaymentElement
} from "@stripe/stripe-js";
import {fetch} from '@tauri-apps/plugin-http';
import {info} from "@tauri-apps/plugin-log";
import type {StripeElementsOptionsMode} from "@stripe/stripe-js/dist/stripe-js/elements-group";

export enum Status {
    Canceled = "Canceled",
    Processing = "Processing",
    RequiresAction = "RequiresAction",
    RequiresCapture = "RequiresCapture",
    RequiresConfirmation = "RequiresConfirmation",
    RequiresPaymentMethod = "RequiresPaymentMethod",
    Succeeded = "Succeeded"
}

interface PaymentResponse {
    clientSecret: string;
    pubKey: string;
    status: Status,
}


async function fetchPayment(email: string | null, deviceId: string): Promise<PaymentResponse> {
    const formData = new FormData();
    if (email != null) {
        formData.append('email', email);
    }

    const response = await fetch(`http://drinknow.test:8080/pricing/payment/setup/${deviceId}`, {
        method: 'POST',
        body: formData
    });

    if (!response.ok) {
        throw new Error(`Can't access payment network: ${response.statusText}`);
    }

    return await response.json() as PaymentResponse;
}

export interface PaymentRequirePaymentMethodSetup {
    elements: StripeElements,
    paymentElement: StripePaymentElement,
    status: Status.RequiresPaymentMethod
}

export interface PaymentSuccess {
    status: Status.Succeeded | Status.RequiresCapture
}

export type StripeResult = PaymentRequirePaymentMethodSetup | PaymentSuccess

export interface StripeSetup {
    stripe: Stripe,
    paymentResult: StripeResult
}


export async function fetchAndInitStripe(email: string | null, deviceId: string): Promise<StripeSetup> {
    const payment = await fetchPayment(email, deviceId)
    const stripe: Stripe | null = await loadStripe(payment.pubKey)
    if (stripe) {
        return {
            stripe: stripe,
            paymentResult: await processPaymentIntent(stripe, payment)
        }
    } else {
        throw new Error("Failed to load Stripe. loadStripe returned null.");
    }
}

async function processPaymentIntent(stripe: Stripe, payment: PaymentResponse): Promise<PaymentRequirePaymentMethodSetup | PaymentSuccess> {
    switch (payment.status) {
        case Status.RequiresPaymentMethod:
            const options: StripeElementsOptionsClientSecret = {
                clientSecret: payment.clientSecret,
                appearance: {
                    theme: 'stripe',
                    disableAnimations: true,
                    variables: {
                        fontFamily: "Switzer",
                        colorBackground: "#f2f2f2",
                        colorText: "#023047",
                        colorTextPlaceholder: "#99A1AF"
                    },
                    rules: {
                        '.Label': {
                            color: '#8ECAE6'
                        }
                    }
                }
            }
            const elements = stripe.elements(options)
            const paymentElement = elements.create("payment", {
                layout: 'tabs',
            })
            return {
                elements: elements,
                paymentElement: paymentElement,
                status: Status.RequiresPaymentMethod
            }
        case Status.Succeeded:
            return {
                status: Status.Succeeded
            }
        case Status.RequiresCapture:
            return {
                status: Status.RequiresCapture
            }
        default:
            throw new Error(`Unhandled payment status: ${payment.status}`);
    }
}