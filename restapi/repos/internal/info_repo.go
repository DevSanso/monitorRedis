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