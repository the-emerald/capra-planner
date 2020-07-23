export enum ZHLSubtype {
    B = "B",
    C = "C"
}

export function ZHLSubtypeFromString(s: string): ZHLSubtype {
    if (s == "B") {
        return ZHLSubtype.B
    }
    else if (s == "C") {
        return ZHLSubtype.C
    }
    else {
        console.log("Unable to convert ZHL Subtype from string!");
        return ZHLSubtype.B; // Meh.
    }
}

export interface ZHLSettings {
    subtype: ZHLSubtype;
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