package internal

import (
	"database/sql"

	"restapi/types/repo_vo"
)

func ClientRepoGenClientList(rows *sql.Rows) ([]repo_vo.ClientInfoVO, error) {
	ret := make([]repo_vo.ClientInfoVO, 0)

	for rows.Next() {

		err := rows.Scan()
		if err != nil {
			return nil, err
		}
	}
	return ret,nil
}