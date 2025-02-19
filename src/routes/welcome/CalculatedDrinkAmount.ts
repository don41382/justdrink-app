import type {GenderType} from "../../bindings";

export interface CalculatedDrinkAmount {
    min: number,
    max: number,
    selected: number
}

export namespace CalculatedDrinkAmount {

    export function calc(gender: GenderType, weightInKg: number) {
        switch (gender) {
            case "Male":
                return weightInKg * 35
            case "Female":
                return weightInKg * 31
            case "Other":
                return weightInKg * 32
        }
    }
}