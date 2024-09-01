export interface IWorkerCtl<T> {
    signal(value : Signal) : Promise<void>
    get() : Promise<RecvData<T> | null>
}

export enum SingalHeader {
    SET_DATA,
    STOP_WORKER
}

export type Signal = {
    header : SingalHeader,
    value : any
}

export type RecvData<T> = {
    seq : number,
    value : T
}

export interface IWebWorker{
    start() : void
}