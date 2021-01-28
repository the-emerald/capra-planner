import {GeneralSettings, ZHLSettings} from "@/common/serde/settings";
import {DiveSegment} from "@/common/serde/dive_segment";
import {Gas} from "@/common/serde/gas";

export interface Dive {
    user: number;
    timestamp: Date;
    zhlSettings: ZHLSettings;
    generalSettings: GeneralSettings;
    segments: Array<[DiveSegment, Gas]>;
    decoGases: Array<Gas>;
}