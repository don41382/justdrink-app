import {Sip} from "./SipSize";
import {sessionTimes} from "../session-times";
import type {SipSize} from "../../bindings";


export namespace DrinkTimeCalculator {
    export function calc(drinkAmountMl: number, selectedSipSize: SipSize): number {
        return roundToNearestSessionTime((12 * 60) / (drinkAmountMl / Sip.getMlForSize(selectedSipSize)))
    }

    function roundToNearestSessionTime(num: number): number {
        let closest = sessionTimes[0];

        // Iterate through sessionTimes to find the closest number
        for (const time of sessionTimes) {
            if (Math.abs(time - num) < Math.abs(closest - num)) {
                closest = time;
            }
        }

        return closest;
    }
}