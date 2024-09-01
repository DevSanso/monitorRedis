import {get_unix_epoch} from './get_unix_epoch';
import {millis_sleep} from './sleep';

export default {
    current_unix_epoch : () => get_unix_epoch(Date.now()),
    sleep : async (second : number) => millis_sleep(second * 1000)
};