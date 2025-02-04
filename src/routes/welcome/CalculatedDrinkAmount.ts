import {GenderType} from "./Gender";

export interface CalculatedDrinkAmount {
    min: number,
    max: number,
    selected: number
}

export namespace CalculatedDrinkAmount {

    export function calc(gender: GenderType, weightInKg: number) {
        switch (gender) {
            case GenderType.Male:
                return weightInKg * 35
            case GenderType.Female:
                return weightInKg * 31
            case GenderType.Other:
                return weightInKg * 32
        }
    }
}