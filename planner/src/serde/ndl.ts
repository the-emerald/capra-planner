import {AxiosResponse} from "axios";

export interface NDLInput {
    depth: number;
}

export interface NDLOutput {
    decoReached: boolean;
    infinite: boolean;
    timeRemaining: number;
}

export function ndlFromResponse(response: AxiosResponse): NDLOutput {
    return {
        decoReached: response.data.deco_reached,
        infinite: response.data.infinite,
        timeRemaining: response.data.time_remaining
    }
}