export enum ZHLVariant {
    B = "B",
    C = "C"
}

export function ZHLSubtypeFromString(s: string): ZHLVariant {
    if (s == "B") {
        return ZHLVariant.B
    }
    else if (s == "C") {
        return ZHLVariant.C
    }
    else {
        console.log("Unable to convert ZHL Subtype from string!");
        return ZHLVariant.B; // Meh.
    }
}

export interface ZHLSettings {
    variant: ZHLVariant;
    gfl: number;
    gfh: number;
}

// eslint-disable-next-line
export interface VPMSettings {
    // Left blank intentionally
}

export interface GeneralSettings {
    sac_bottom: number;
    sac_deco: number;
    ascent_rate: number;
    descent_rate: number;
}