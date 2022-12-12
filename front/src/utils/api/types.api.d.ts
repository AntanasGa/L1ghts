export interface StepGet {
    step: "installed" | "setup",
}

export interface AuthReqPost {
    user_name: string,
    password: string,
}

export interface AuthPost {
    access_token: string,
    refresh_token: string,
}

export interface Devices {
    id: number,
    adr: number,
    pairs_of: number,
    endpoint_count: number,
}

export interface Points {
    id: number,
    device_id: number,
    val: number,
    width: number,
    height: number,
    x: number,
    y: number,
    rotation: number,
    watts: number,
    active: boolean,
    tag: string | null,
}

export interface QueryById {
    id: number,
}

export interface Presets extends NewPresets, QueryById {
}

export interface NewPresets {
    preset_name: string,
    favorite: boolean,
    icon: string | null,
}
