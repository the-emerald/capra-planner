import axios, {AxiosResponse} from 'axios';
import {User} from "@/common/serde/user";
import {GeneralSettings, ZHLSettings} from "@/common/serde/settings";

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
            "new_zhl_settings": settings
        }
    )
}

export function updateGeneralSettings(user: User, settings: GeneralSettings): Promise<AxiosResponse> {
    return axios.post(
        '/settings/update/general',
        {
            "id": user.id,
            "new_general_settings": settings
        }
    )
}