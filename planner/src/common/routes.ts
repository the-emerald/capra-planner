import axios, {AxiosResponse} from 'axios';

export function newUser(name: string): Promise<AxiosResponse> {
    // Make request to server
    return axios.post(
        '/user/new',
        {
            name: name
        },
    )
}

export function aboutUser(id: number): Promise<AxiosResponse> {
    return axios.post(
        '/user',
        {
            id: id
        }
    )
}

export function listAllUsers(): Promise<AxiosResponse> {
    return axios.post(
        '/user/all',
    )
}