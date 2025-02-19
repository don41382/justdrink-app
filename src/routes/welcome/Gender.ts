import type {GenderImages} from "./+page";
import type {GenderType} from "../../bindings";

export let allGender: GenderType[] = ["Male", "Female", "Other"]

export function imagePath(gender: GenderType, genderImages: GenderImages) {
    switch (gender) {
        case "Male":
            return genderImages.male
        case "Female":
            return genderImages.female
        case "Other":
            return genderImages.other
    }
}
