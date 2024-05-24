package repo_vo


type ClientInfoVO struct {
    CollectTime string
    ID          int64
    Addr        string
    FD          int64
    Name        string
    Age         int64
    Idle        int64
    Flags       string
    DB          int64
    Sub         int64
    PSub        int64
    Multi       int64
    QBuf        int64
    QBufFree    int64
    OBL         int64
    OLL         int64
    OMem        int64
    Events      string
    Cmd         string
}
