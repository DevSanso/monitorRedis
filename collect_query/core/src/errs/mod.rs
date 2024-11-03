use crate::macros::impl_err_mod;

impl_err_mod!(data, [
    (GetDataCastError, "maybe not matching wanted cast type to data", "row data cast internal error"),
    (CantSplitTupleError, "can't data split tuple data", "maybe split data in string, check raw data"),
    (CantMappingKeyError, "unsupport select this key data", "not handling this key select, add key handling"),
    (EncodingCastError, "string encoding casting failed", "check data string encoding type")
]);

impl_err_mod!(connection, [
    (GetConnectionFailedError, "get other process connection", "check other process state"),
    (NotMatchArgsLenError, "query bound args count not mathcing", "query parameter length not maching, check bound varibles"),
    (ResponseScanError, "connection response data read error", "check server state or error handling code"),
    (CommandRunError, "running command or query is error", "check query or command"),
    (ConnectionApiCallError, "connection api function return error", "check server env or process state")
]);

impl_err_mod!(fetch, [
    (RowIdxNotExistError, "not exists rows dataset, this idx key", "maybe not exists this idxs key in rows dataset"),
    (NilDataError, "database return data is none data", "database return data is none, this error not critial error"),
    (GetFailedError, "get database query failed", "check query or fetch rust function code")
]);

impl_err_mod!(conf, [
    (ProcessConfigPathError, "need process config file path", "plz input  config path when run commmand, (ex: collect_server ../config.json)")
]);

impl_err_mod!(proc, [
    (CriticalError, "critial error", "can't kown error, plz check process"),
    (GenResultIsNoneError, "gen function is return none", "if pool gen function errors, data is none check process"),
    (MaxSizedError, "memeory or pool is used Max size", "can't alloc new memory"),
    (PoolNotSetError, "pool state is not init", "check code, create pool function"),
    (PoolGetItemError, "failed pool item", "mayby pool ls used max or gen function failed, check env"),
    (NoneDataError, "none data", "used var is not exists data, check code"),
    (RootError, "errors list", "it error use utils_inherit_error"),
    (UnkownError, "", "")
]);

