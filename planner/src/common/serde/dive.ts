import {PlanType} from "@/common/serde/plan_type";
import {GeneralSettings, ZHLSettings} from "@/common/serde/settings";
import {DiveSegment} from "@/common/serde/dive_segment";
import {Gas} from "@/common/serde/gas";

export interface Dive {
    user: number;
    planType: PlanType;
    // TODO: how to handle tissue?
    surfaceInterval: number;
    // TODO: how to handle datetime?
    zhlSettings: ZHLSettings;
    generalSettings: GeneralSettings;
    segments: Array<[DiveSegment, Gas]>;
    decoGases: Array<Gas>;
}