import _utils, {UtilsLib} from './utils/lib';
import _worker, {WorkerLib} from './worker/lib';

export const utils = _utils;
export const worker = _worker;


type CoreLib = {
    utils :UtilsLib;
    worker : WorkerLib;
}

declare global {
    export const monCore : CoreLib;
}