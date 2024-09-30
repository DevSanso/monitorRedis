package internal

import (
	"database/sql"

	"restapi/types/repo_vo"
)
//collect_time, id, addr, fd, name, age, idle, flags, db, sub, psub, multi, qbuf, qbuf_free, obl, oll, omem, events, cmd 
func DbRepoGenKeyTopUsage(rows *sql.Rows) ([]repo_vo.DbKeyMemUsage, error) {
	ret := make([]repo_vo.DbKeyMemUsage, 0)

	for rows.Next() {
		temp := repo_vo.DbKeyMemUsage{}
		
		err := rows.Scan(&temp.CollectTime, &temp.KeyName, &temp.UsageByte, &temp.ExpiredSec)
		
		if err != nil {
			return nil, err
		}

		ret = append(ret, temp)
	}
	return ret,nil
}

func DbRepoGenKeySpaceInfo(rows *sql.Rows) ([]repo_vo.DbKeySpaceInfo, error) {
	ret := make([]repo_vo.DbKeySpaceInfo, 0)

	for rows.Next() {
		temp := repo_vo.DbKeySpaceInfo{}
		
		err := rows.Scan(&temp.DbName, &temp.CollectTime, &temp.Keys, &temp.Expired, &temp.AvgTTL)
		
		if err != nil {
			return nil, err
		}

		ret = append(ret, temp)
	}
	return ret,nil
}