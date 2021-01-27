import axios, {AxiosResponse} from 'axios';
import {User} from "@/common/serde/user";
import {GeneralSettings, ZHLSettings} from "@/common/serde/settings";
import {DiveSegment} from "@/common/serde/dive_segment";
import {Gas} from "@/common/serde/gas";
import {PlanType} from "@/common/serde/plan_type";
import {Algorithm} from "@/common/serde/algorithm";

export function newUser(name: string): Promise<AxiosResponse> {
    return axios.post(
        '/user/new',
        {
            name: name
        },
    )
}

export function getUser(id: number): Promise<AxiosResponse> {
    return axios.post(
        '/user',
        {
            id: id
        }
    )
}

export function listAllUsers(): Promise<AxiosResponse<Array<User>>> {
    return axios.post(
        '/user/all',
    )
}

export function updateZHLSettings(user: User, settings: ZHLSettings): Promise<AxiosResponse> {
    return axios.post(
        '/settings/update/zhl',
        {
            "id": user.id,
            "newZhlSettings": settings
        }
    )
}

export function updateGeneralSettings(user: User, settings: GeneralSettings): Promise<AxiosResponse> {
    return axios.post(
        '/settings/update/general',
        {
            "id": user.id,
            "newGeneralSettings": settings
        }
    )
}

export function getDivePlan(user: User, diveType: PlanType, algorithm: Algorithm, surfaceInterval: number, segments: Array<[DiveSegment, Gas]>, decoGases: Array<Gas>):
    Promise<AxiosResponse<PlanDiveResponse>> {

    return axios.post(
        '/dive/',
        {
            "id": user.id,
            "planType": diveType,
            "surfaceInterval": surfaceInterval,
            "algorithm": algorithm,
            "parameters": {
                "segments": segments,
                "decoGases": decoGases
            }
        }
    )
}

// export function planDive(user: User, algorithm: Algorithm, surfaceInterval: number, segments: Array<[DiveSegment, Gas]>, decoGases: Array<Gas>): Promise<AxiosResponse> {
//     return getDivePlan(user, PlanType.PLAN, algorithm, surfaceInterval, segments, decoGases)
// }
//
// export function executeDive(user: User, algorithm: Algorithm, surfaceInterval: number, segments: Array<[DiveSegment, Gas]>, decoGases: Array<Gas>): Promise<AxiosResponse> {
//     return getDivePlan(user, PlanType.EXECUTE, algorithm, surfaceInterval, segments, decoGases)
// }

export interface PlanDiveResponse {
    segments: Array<[DiveSegment, Gas]>;
    gasUsed: Array<[Gas, number]>;
}