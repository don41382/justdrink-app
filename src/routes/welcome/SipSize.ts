import type {SipSize} from "../../bindings";

export namespace Sip {

    export interface Data {
        type: SipSize,
        ml: number,
        text: string,
        description: string,
    }

    export const sizes: Data[] = [{
        type: "FullCup",
        ml: 330,
        text: "1 Glass",
        description: "a glass",
    }, {
        type: "HalfCup",
        ml: 165,
        text: "1/2 Glass",
        description: "half a glass",
    }, {
        type: "BigSip",
        ml: 15*3,
        text: "a big sip",
        description: "a big sip",
    }]

    export function getMlForSize(sipSize: SipSize): number {
        const ml = sizes.find((s) => {return s.type === sipSize})?.ml
        return ml ?? 100
    }

    export function getTextForSize(sipSize: SipSize): string | undefined {
        return sizes.find((s) => {return s.type == sipSize})?.description
    }

}