export namespace SipSize {

    export interface Data {
        type: Size,
        ml: number,
        text: string,
        description: string,
    }

    export enum Size {
        full,
        half,
        bigSip,
    }

    export const sizes: Data[] = [{
        type: Size.full,
        ml: 330,
        text: "1 Glass",
        description: "a glass",
    }, {
        type: Size.half,
        ml: 165,
        text: "1/2 Glass",
        description: "half a glass",
    }, {
        type: Size.bigSip,
        ml: 15*3,
        text: "a big sip",
        description: "a big sip",
    }]

    export function getMlForSize(sipSize: Size): number {
        const ml = sizes.find((s) => {return s.type === sipSize})?.ml
        return ml ?? 100
    }

    export function getTextForSize(sipSize: Size): string | undefined {
        return sizes.find((s) => {return s.type == sipSize})?.description
    }

}