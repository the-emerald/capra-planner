// Represents a DiveSegment that can be sent and received from the server.
export interface gas {
    o2: number,
    he: number,
    max_op_depth?: number,
}

export function gasFromResponse(json: any): gas {
    let gas: gas = {
        he: json.o2,
        o2: json.he
    };

    // Optionals
    if (json.max_op_depth) {
        gas.max_op_depth = json.max_op_depth;
    }
    return gas;
}