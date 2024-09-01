import * as typed from './types';
import * as ctl from './web_controller';


export type WorkerCtl<T> = typed.IWorkerCtl<T>;
export const CreateWorkerCtl = ctl.ImplWorkerCtl;

