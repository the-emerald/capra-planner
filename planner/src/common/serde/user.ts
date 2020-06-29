export interface User {
    id: number;
    name: string;
}

// eslint-disable-next-line
export function userFromResponse(json: any): User {
    return {
        id: json.id,
        name: json.name
    }
}