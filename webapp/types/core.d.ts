declare module "utils/get_unix_epoch" {
    export const get_unix_epoch: (time: number) => number;
}
declare module "utils/sleep" {
    export const millis_sleep: (milliseconds: number) => Promise<void>;
}
declare module "utils/lib" {
    export type UtilsLib = {
        current_unix_epoch: () => number;
        sleep: (second: number) => Promise<void>;
    };
    const _default: {
        current_unix_epoch: () => number;
        sleep: (second: number) => Promise<void>;
    };
    export default _default;
}
declare module "worker/types" {
    export interface IWorkerCtl<T> {
        signal(value: Signal): Promise<void>;
        get(): Promise<RecvData<T> | null>;
    }
    export enum SingalHeader {
        SET_DATA = 0,
        STOP_WORKER = 1
    }
    export type Signal = {
        header: SingalHeader;
        value: any;
    };
    export type RecvData<T> = {
        seq: number;
        value: T;
    };
    export interface IWebWorker {
        start(): void;
    }
}
declare module "worker/web_controller" {
    import * as t from "worker/types";
    export const ImplWorkerCtl: <T>(name: string, script_path: string) => t.IWorkerCtl<T>;
}
declare module "worker/lib" {
    import * as typed from "worker/types";
    export type WorkerLib = {
        CreateWorkerCtl: <T>(name: string, script_path: string) => typed.IWorkerCtl<T>;
    };
    export const typedef: typeof typed;
    const _default_1: {
        CreateWorkerCtl: <T>(name: string, script_path: string) => typed.IWorkerCtl<T>;
    };
    export default _default_1;
}
declare module "index" {
    import { UtilsLib } from "utils/lib";
    import { WorkerLib } from "worker/lib";
    export const utils: {
        current_unix_epoch: () => number;
        sleep: (second: number) => Promise<void>;
    };
    export const worker: {
        CreateWorkerCtl: <T>(name: string, script_path: string) => import("worker/types").IWorkerCtl<T>;
    };
    type CoreLib = {
        utils: UtilsLib;
        worker: WorkerLib;
    };
    global {
        export const monCore: CoreLib;
    }
}
declare module "worker/web_worker" {
    import { IWebWorker } from "worker/types";
    export const WeSubbWorker: (interval_sec: number, callback: Function) => IWebWorker;
}
