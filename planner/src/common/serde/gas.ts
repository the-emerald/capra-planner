// Represents a DiveSegment that can be sent and received from the server.
export interface Gas {
    o2: number;
    he: number;
    maxOpDepth?: number;
}

export function gasFromResponse(json: any): Gas {
    const gas: Gas = {
        he: json.o2,
        o2: json.he
    };

    // Optionals
    if (json.max_op_depth) {
        gas.maxOpDepth = json.max_op_depth;
    }
    return gas;
}