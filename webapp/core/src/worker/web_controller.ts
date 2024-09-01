import * as t from './types';

export const ImplWorkerCtl = <T>(name : string, script_path : string) : t.IWorkerCtl<T> =>   {
    let buffer : t.RecvData<T> | null = null;
    let change_flag = new Uint8Array(new SharedArrayBuffer(0));
    let worker : Worker = new Worker(script_path);

    worker.onmessage = async (event : MessageEvent<t.RecvData<T>>) => {
        await navigator.locks.request(name, (l) => {
            buffer = event.data;
        });
    };

    return {
        signal : async (value :t.Signal)  => {
            worker.postMessage(value);
        },
        get : async() : Promise<t.RecvData<T> | null> => {
            let data : t.RecvData<T> | null = null;
            await navigator.locks.request(name, (l) => { data = buffer; });

            return data;
        }
    };
}