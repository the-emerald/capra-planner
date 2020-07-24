import {Vue} from "vue/types/vue";

export function makeErrorToast(component: Vue, message: string) {
    component.$bvToast.toast(
        message,
        {
            title: "Error",
            variant: "danger",
            toaster: "b-toaster-top-center",
            solid: true
        }
    )
}

export function makeSuccessToast(component: Vue, message: string) {
    component.$bvToast.toast(
        message,
        {
            title: "Success",
            variant: "success",
            toaster: "b-toaster-top-center",
            solid: true
        }
    )
}