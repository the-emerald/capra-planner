import {diveSegment} from "@/common/dive_segment";
import {gas} from "@/common/gas";
import {prettyFromMilliseconds} from "@/common/time";

export function displayDiveSegmentGas(input: [diveSegment, gas]): string {
    return `${input[0].startDepth}m | ${prettyFromMilliseconds(input[0].time)} | ${input[1].o2}/${input[1].he}`

}