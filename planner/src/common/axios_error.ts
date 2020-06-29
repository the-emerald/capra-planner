// eslint-disable-next-line
export function handleAxiosError(error: any): string {
    if (error.response) {
        return "Server encountered an error."
    }
    else if (error.request) {
        return "Could not contact server."
    }
    else {
        return "Please complain to Emerald about it."
    }
}