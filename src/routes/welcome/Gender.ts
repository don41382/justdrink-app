import type {GenderImages} from "./+page";

export enum GenderType {
    Male = "Male",
    Female = "Female",
    Other = "Other"
}

export function imagePath(gender: GenderType, genderImages: GenderImages) {
    switch (gender) {
        case GenderType.Male: return genderImages.male
        case GenderType.Female: return genderImages.female
        case GenderType.Other: return genderImages.other
    }
}
