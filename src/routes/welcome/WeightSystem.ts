import {MeasureSystem} from "./MeasureSystem";

export enum WeightSystem {
    kg = "kg",
    lbs = "lbs"
}

export function getWeightSystem(system: MeasureSystem) {
    switch (system) {
        case MeasureSystem.Metric:
            return WeightSystem.kg
        case MeasureSystem.Imperial:
            return WeightSystem.lbs
    }
}

export function getMeasureSystem(weightSystem: WeightSystem): MeasureSystem {
    switch (weightSystem) {
        case WeightSystem.kg:
            return MeasureSystem.Metric
        case WeightSystem.lbs:
            return MeasureSystem.Imperial
    }
}