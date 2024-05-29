package repo_vo

type InfoCpuVO struct {
	CollectTime string
	CpuSys float64
	CpuUser float64
	ChildCpuSys float64
	ChildCpuUser float64
}

type InfoStatVO struct {
	CollectTime string
    TotalConnectionsReceived int64
    TotalCommandsProcessed   int64
    InstantaneousOpsPerSec   int64
    TotalNetInputBytes       int64
    TotalNetOutputBytes      int64
    InstantaneousInputKbps   float64
    InstantaneousOutputKbps  float64
    RejectedConnections      int64
    SyncFull                int64
    SyncPartialOK           int64
    SyncPartialErr          int64
    ExpiredKeys             int64
    ExpiredStalePerc        float64
    ExpiredTimeCapReachedCount int64
    EvictedKeys             int64
    KeyspaceHits            int64
    KeyspaceMisses          int64
    PubsubChannels           int64
    PubsubPatterns          int64
    LatestForkUsec           int64
    MigrateCachedSockets    int64
    SlaveExpiresTrackedKeys int64
    ActiveDefragHits        int64
    ActiveDefragMisses      int64
    ActiveDefragKeyHits     int64
    ActiveDefragKeyMisses   int64
}