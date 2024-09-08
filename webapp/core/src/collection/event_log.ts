import * as t from './types';

export const NewEventLog = <T extends Object>(size : number, storage : Storage | null): t.Logger<t.TimeItem<T>, T> => {
    const buffer : Array<t.TimeItem<T>> = new Array(size);
    const eventLogIdent = crypto.randomUUID();
    let currentIdx = 0;
    
    return {
        get_logs : async (count : number)  => {
            return await navigator.locks.request(eventLogIdent, (l) => {
                let temp = buffer.slice(0, count);
                return temp;
            });
        },
        push : async (item : T) : Promise<void> => {
            await navigator.locks.request(eventLogIdent, (l) => {
                if(buffer.length >= currentIdx) {
                    let f = buffer.shift();

                    if (storage != null) storage.removeItem(`${eventLogIdent}:${f?.pushTime}`);
                    
                }
                let t = Date.now();
                buffer.push({
                    item : item,
                    pushTime : t
                });

                currentIdx += 1;
                
                if (storage != null) storage.setItem(`${eventLogIdent}:${t}`, typeof item === 'string' ? item  as string: item.toString());             
            });
        }
    };
};