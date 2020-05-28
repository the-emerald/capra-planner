import {AxiosResponse} from "axios";

export class NDLInput {
    depth: number;

    constructor(depth: number) {
        this.depth = depth;
    }

    send() {
        return {
            depth: Number(this.depth)
        }
    }
}

export class NDLOutput {
    public decoReached: boolean;
    public infinite: boolean;
    public timeRemaining: number;

    constructor(response: AxiosResponse) {
        this.decoReached = response.data.deco_reached;
        this.infinite = response.data.infinite;
        this.timeRemaining = response.data.time_remaining
    }
}