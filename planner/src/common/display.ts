import {prettyFromMilliseconds} from "@/common/time";
import {BottomSegmentElement, DecoGasElement} from "@/store/plan";
import {Gas} from "@/common/serde/gas";

export function displayGas(gas: Gas): string {
    return `${gas.o2}/${gas.he}`
}

export function displayBottomSegmentElement(input: BottomSegmentElement): string {
    return `${input.diveSegment.startDepth}m | ${prettyFromMilliseconds(input.diveSegment.time)} | ${displayGas(input.gas)}`
}

export function displayDecoGasElement(input: DecoGasElement): string {
    if (input.gas.maxOperatingDepth) { // If MOD exists
        return `${displayGas(input.gas)} (${input.gas.maxOperatingDepth}m)`
    }
    else {
        return displayGas(input.gas)
    }
}