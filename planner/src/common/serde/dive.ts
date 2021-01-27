import {PlanType} from "@/common/serde/plan_type";
import {GeneralSettings, ZHLSettings} from "@/common/serde/settings";
import {DiveSegment} from "@/common/serde/dive_segment";
import {Gas} from "@/common/serde/gas";
import {Tissue} from "@/common/serde/tissue";

export interface Dive {
    user: number;
    planType: PlanType;
    tissue: Tissue;
    surfaceInterval: number;
    timestamp: Date;
    zhlSettings: ZHLSettings;
    generalSettings: GeneralSettings;
    segments: Array<[DiveSegment, Gas]>;
    decoGases: Array<Gas>;
}