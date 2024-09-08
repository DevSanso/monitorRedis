import * as typed from './types';
import * as ctl from './web_controller';

export type WorkerLib = {
    CreateWorkerCtl :<T>(name : string, script_path : string) => typed.IWorkerCtl<T>
};

export const typedef = typed;

export default {
    CreateWorkerCtl : ctl.ImplWorkerCtl
}


