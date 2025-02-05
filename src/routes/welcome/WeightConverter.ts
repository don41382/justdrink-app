import {MeasureSystem} from "./MeasureSystem";
import {GenderType} from "./Gender";

export class WeightConverter {
    private static readonly KG_TO_LBS = 2.20462;
    private static readonly LBS_TO_KG = 1 / WeightConverter.KG_TO_LBS;

    /**
     * Converts a value from the given measure system to kilograms.
     * @param value - The weight value in the specified measure system.
     * @param system - The system of the given value (Metric or Imperial).
     * @returns The weight in kilograms as a neutral number.
     */
    static from(value: number, system: MeasureSystem): number {
        return Number((system === MeasureSystem.Metric
            ? value
            : value * this.LBS_TO_KG).toFixed(0));
    }

    /**
     * Converts a value to the given measure system from kilograms.
     * @param value - The weight in kilograms.
     * @param system - The target system (Metric or Imperial).
     * @returns The converted weight as a neutral number.
     */
    static to(value: number, system: MeasureSystem): number {
        return Number((system === MeasureSystem.Metric
            ? value
            : value * this.KG_TO_LBS).toFixed(0));
    }

    static toWeightName(measureSystem: MeasureSystem) {
        switch (measureSystem) {
            case MeasureSystem.Metric: return "kg"
            case MeasureSystem.Imperial: return "lb"
        }
    }

    static defaultWeightByGender(gender: GenderType): number {
        switch (gender) {
            case GenderType.Male: return 75
            case GenderType.Female: return 65
            case GenderType.Other: return 70
        }
    }

}