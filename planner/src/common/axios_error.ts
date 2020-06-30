// eslint-disable-next-line
export function handleAxiosError(error: any): string {
    if (error.response) {
        return `Server encountered an error. (${error.response.status})`
    }
    else if (error.request) {
        return "Could not contact server. Please make sure it is reachable."
    }
    else {
        return "Something went seriously wrong. Please complain to Emerald about this."
    }
}