import {extend} from "vee-validate"
import {required} from "vee-validate/dist/rules";
import {User} from "@/common/serde/user";

extend('positive', value => {
    return Number(value) >= 0;
});

extend('required', required);

extend('gas', {
    params: ['other'],
    // eslint-disable-next-line
    validate(value, x: any) {
        // This is done so that edits (which are strings) are checked correctly
        return (Number(value) <= 100) && (Number(value) + Number(x.other) <= 100)
    }
});

extend('lt', {
    params: ['val'],
    // eslint-disable-next-line
    validate(value, x: any) {
        return Number(value) < Number(x.val)
    }
});

extend('eq', {
    params: ['val'],
    // eslint-disable-next-line
    validate(value, x: any) {
        return Number(value) == Number(x.val)
    }
});

extend('gt', {
    params: ['val'],
    // eslint-disable-next-line
    validate(value, x: any) {
        return value > x.val
    }
});

extend('uniqueName', {
    params: ['val'],
    // eslint-disable-next-line
    validate(value, x: any) {
        console.log(value);
        const ns = x.val.map(
            function (value: User) {
                return value.name;
            }
        );

        console.log(ns);
        return !ns.includes(value)
    }
});