export type Pain = { id: PainId, title: string }
export type PainId = 'neck-upper-back' | 'lower-back' | 'wrist-hand' | 'stress';

export let pains: Pain[] = [{
    id: "neck-upper-back",
    title: "Neck & Upper Back",
}, {
    id: "lower-back",
    title: "Lower Back",
}, {
    id: "wrist-hand",
    title: "Wrist & Hand",
}, {
    id: "stress",
    title: "Stress",
}]
