import {extend} from "vee-validate"
import {required} from "vee-validate/dist/rules";

extend('positive', value => {
    return value >= 0;
});

extend('required', required);

extend('gas', {
    params: ['other'],
    // Just shut up!
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
        return value < x.val
    }
});

extend('eq', {
    params: ['val'],
    // eslint-disable-next-line
    validate(value, x: any) {
        return value == x.val
    }
});

extend('gt', {
    params: ['val'],
    // eslint-disable-next-line
    validate(value, x: any) {
        return value > x.val
    }
});