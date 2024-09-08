export type Deque<T> = {
    pop : (timeout : number) => Promise<T | null>;
};

export type Queue<OUTPUT, INPUT> = {
    pop : (timeout : number) => Promise<OUTPUT | null>;
    push : (item : INPUT) => Promise<void>; 
};

export type Logger<OUTPUT, INPUT> = {
    get_logs : (count : number) => Promise<Array<OUTPUT> | null>;
    push : (item : INPUT) => Promise<void>; 
};

export type TimeItem<T> = {
    item : T ;
    pushTime : number;
}