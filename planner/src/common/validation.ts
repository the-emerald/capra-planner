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
        return (value <= 100) && (value + x.other <= 100)
    }
});

extend('max', {
    params: ['rhs'],
    // eslint-disable-next-line
    validate(value, x: any) {
        return value < x.rhs
    }
});