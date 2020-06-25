import {prettyFromMilliseconds} from "@/common/time";
import {BottomSegmentElement, DecoGasElement} from "@/store/plan";

export function displayBottomSegmentElement(input: BottomSegmentElement): string {
    return `${input.diveSegment.startDepth}m | ${prettyFromMilliseconds(input.diveSegment.time)} | ${input.gas.o2}/${input.gas.he}`
}

export function displayDecoGasElement(input: DecoGasElement): string {
    if (input.gas.maxOpDepth) { // If MOD exists
        return `${input.gas.o2}/${input.gas.he} (${input.gas.maxOpDepth}m)`
    }
    else {
        return `${input.gas.o2}/${input.gas.he}`
    }
}