package internal

import (
	"database/sql"

	"restapi/types/repo_vo"
)

func ServerRepoGenInfoCpuList(rows *sql.Rows) ([]repo_vo.ServerCpuVO, error) {
	ret := make([]repo_vo.ServerCpuVO, 0)

	for rows.Next() {
		temp := repo_vo.ServerCpuVO{}

		err := rows.Scan(&temp.CollectTime, &temp.CpuSys, &temp.CpuUser, &temp.ChildCpuSys, &temp.ChildCpuUser, &temp.UptimeSecond)
		if err != nil {
			return nil, err
		}
		ret = append(ret, temp)
	}
	return ret, nil
}

func ServerRepoGenInfoStats(row *sql.Row) (*repo_vo.ServerStatVO, error) {
	buf := new(repo_vo.ServerStatVO)

	err := row.Scan(
		&buf.CollectTime,
		&buf.TotalConnectionsReceived,
		&buf.TotalCommandsProcessed,
		&buf.InstantaneousOpsPerSec,
		&buf.TotalNetInputBytes,
		&buf.TotalNetOutputBytes,
		&buf.InstantaneousInputKbps,
		&buf.InstantaneousOutputKbps,
		&buf.RejectedConnections,
		&buf.SyncFull,
		&buf.SyncPartialOK,
		&buf.SyncPartialErr,
		&buf.ExpiredKeys,
		&buf.EvictedKeys,
		&buf.KeyspaceHits,
		&buf.KeyspaceMisses,
		&buf.PubsubChannels,
		&buf.PubsubPatterns,
		&buf.LatestForkUsec,
		&buf.MigrateCachedSockets,
		&buf.SlaveExpiresTrackedKeys,
		&buf.ActiveDefragHits,
		&buf.ActiveDefragMisses,
		&buf.ActiveDefragKeyHits,
		&buf.ActiveDefragKeyMisses,
	)

	if err != nil {
		return nil, err
	}

	return buf, nil
}
