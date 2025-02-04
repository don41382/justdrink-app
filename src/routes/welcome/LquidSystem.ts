import {MeasureSystem} from "./MeasureSystem";

export enum LiquidSystem {
    ml = "ml",
    oz = "oz"
}

export function getLiquidSystem(system: MeasureSystem): LiquidSystem {
    switch (system) {
        case MeasureSystem.Metric:
            return LiquidSystem.ml;
        case MeasureSystem.Imperial:
            return LiquidSystem.oz;
    }
}

export function getMeasureSystemForLiquid(liquidSystem: LiquidSystem): MeasureSystem {
    switch (liquidSystem) {
        case LiquidSystem.ml:
            return MeasureSystem.Metric;
        case LiquidSystem.oz:
            return MeasureSystem.Imperial;
    }
}