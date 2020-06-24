import {prettyFromMilliseconds} from "@/common/time";
import {BottomSegmentElement} from "@/store/plan";

export function displayBottomSegmentElement(input: BottomSegmentElement): string {
    return `${input.diveSegment.startDepth}m | ${prettyFromMilliseconds(input.diveSegment.time)} | ${input.gas.o2}/${input.gas.he}`
}