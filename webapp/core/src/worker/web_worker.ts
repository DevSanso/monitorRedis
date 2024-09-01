import {IWebWorker} from './types';
import utils from '../utils/lib';

export const WeSubbWorker = (interval_sec : number, callback : Function) : IWebWorker => {
    const _internal_start = () => {
        let cur = utils.current_unix_epoch();
        let set_start_time = cur % interval_sec;
        if (set_start_time != 0 ) {
            set_start_time = interval_sec - set_start_time;
        }
        
        setTimeout(async() => {
            callback();
            _internal_start();
            await utils.sleep(1);
        }, set_start_time);
        
    };

    return {
        start : () => {
            _internal_start();
        }
    }
}