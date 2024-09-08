import * as t from './types';
import * as ev_log from './event_log';


export type CollectionLib = {
    NewEventLog :  <T extends Object>(size : number, storage : Storage | null)=> t.Logger<t.TimeItem<T>, T>;
};

export default {
    NewEventLog : ev_log.NewEventLog,
}