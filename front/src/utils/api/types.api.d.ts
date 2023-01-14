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
    endpoint_count: number,
}
export interface UpdatePoints {
    id: number,
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

export interface Points extends UpdatePoints {
    device_id: number,
    device_position: number,
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
