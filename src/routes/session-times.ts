export const sessionTimes: number[] = [5, 15, 15, 30, 60, 90, 120]

export function formatDuration(duration: number): string {
    if (duration < 120) {
        return `${duration} min`
    } else {
        const hours = Math.floor(duration / 60)
        return `${hours} hour${hours > 1 ? 's' : ''}`
    }
}