import {SegmentType} from "@/common/serde/segment_type"
// Represents a DiveSegment that can be sent and received from the server.
export interface DiveSegment {
    segment_type: SegmentType;
    start_depth: number;
    end_depth: number;
    time: number;
    ascent_rate?: number;
    descent_rate?: number;
}