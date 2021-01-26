import {PlanType} from "@/common/serde/plan_type";
import {GeneralSettings, ZHLSettings} from "@/common/serde/settings";
import {DiveSegment} from "@/common/serde/dive_segment";
import {Gas} from "@/common/serde/gas";

export interface Dive {
    user: number;
    plan_type: PlanType;
    // TODO: how to handle tissue?
    surface_interval: number;
    // TODO: how to handle datetime?
    zhl_settings: ZHLSettings;
    general_settings: GeneralSettings;
    segments: Array<[DiveSegment, Gas]>;
    deco_gases: Array<[Gas, number?]>;
}