import {segmentType} from "@/common/serde/segment_type"
// Represents a DiveSegment that can be sent and received from the server.
export interface DiveSegment {
    segmentType: segmentType;
    startDepth: number;
    endDepth: number;
    time: number;
    ascentRate: number;
    descentRate: number;
}

export function diveSegmentFromResponse(json: any): DiveSegment {
    return {
        segmentType: json.segment_type,
        startDepth: json.start_dpeth,
        endDepth: json.end_depth,
        time: json.time,
        ascentRate: json.ascent_rate,
        descentRate: json.descent_rate
    }
}