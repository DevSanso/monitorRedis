import _utils, {UtilsLib} from './utils/lib';
import _worker, {WorkerLib} from './worker/lib';
import _collection, {CollectionLib} from './collection/lib';

export const utils = _utils;
export const worker = _worker;
export const collection = _collection;

type CoreLib = {
    utils :UtilsLib;
    worker : WorkerLib;
    collection : CollectionLib;
}

declare global {
    export const monCore : CoreLib;
}