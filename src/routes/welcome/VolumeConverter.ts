import { MeasureSystem } from "./MeasureSystem";

export class VolumeConverter {
    private static readonly ML_TO_OZ = 0.033814;
    private static readonly OZ_TO_ML = 1 / VolumeConverter.ML_TO_OZ;

    /**
     * Converts a value from the specified measurement system to milliliters.
     * @param value - The volume value in the given measurement system.
     * @param system - The measurement system (Metric or Imperial).
     * @returns The volume in milliliters.
     */
    static from(value: number, system: MeasureSystem): number {
        return Number((system === MeasureSystem.Metric ? value : value * this.OZ_TO_ML).toFixed());
    }

    /**
     * Converts a value from milliliters to the specified measurement system.
     * @param value - The volume in milliliters.
     * @param system - The target measurement system (Metric or Imperial).
     * @returns The converted volume.
     */
    static to(value: number, system: MeasureSystem): number {
        return Number((system === MeasureSystem.Metric ? value : value * this.ML_TO_OZ).toFixed());
    }
}