export function limitNumber(input: string, max: number): number {
    const n = Number(input.replace(/[^0-9.]/g, ""))
    if (n > max) {
        return Number(input.substring(0,max))
    } else {
        return n
    }
}
