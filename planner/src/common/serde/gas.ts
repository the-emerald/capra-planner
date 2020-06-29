// Represents a DiveSegment that can be sent and received from the server.
export interface Gas {
    o2: number;
    he: number;
    maxOpDepth?: number;
}