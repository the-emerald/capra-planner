import {SegmentType} from "@/common/serde/segment_type"
// Represents a DiveSegment that can be sent and received from the server.
export interface DiveSegment {
    segmentType: SegmentType;
    startDepth: number;
    endDepth: number;
    time: number;
    ascentRate: number;
    descentRate: number;
}