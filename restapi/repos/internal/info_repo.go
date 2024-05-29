package internal

import (
	"database/sql"

	"restapi/types/repo_vo"
)

func InfoRepoGenInfoCpuList(rows *sql.Rows) ([]repo_vo.InfoCpuVO, error) {
	ret := make([]repo_vo.InfoCpuVO, 0)

	for rows.Next() {
		temp := repo_vo.InfoCpuVO{};

		err := rows.Scan(&temp.CollectTime, &temp.CpuSys, &temp.CpuUser, &temp.ChildCpuSys, &temp.ChildCpuUser)
		if err != nil {
			return nil, err
		}
		ret = append(ret, temp)
	}
	return ret,nil
}

func InfoRepoGenInfoStats(row *sql.Row) (*repo_vo.InfoStatVO, error) {
	buf := new(repo_vo.InfoStatVO);

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