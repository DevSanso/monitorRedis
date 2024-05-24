package internal

import (
	"database/sql"

	"restapi/types/repo_vo"
)
//collect_time, id, addr, fd, name, age, idle, flags, db, sub, psub, multi, qbuf, qbuf_free, obl, oll, omem, events, cmd 
func ClientRepoGenClientList(rows *sql.Rows) ([]repo_vo.ClientInfoVO, error) {
	ret := make([]repo_vo.ClientInfoVO, 0)

	for rows.Next() {
		temp := repo_vo.ClientInfoVO{}
		
		err := rows.Scan(&temp.CollectTime, &temp.ID, &temp.Addr, &temp.FD, 
			&temp.Name, &temp.Age, &temp.Idle, &temp.Flags, &temp.DB, &temp.Sub,
		&temp.PSub, &temp.Multi, &temp.QBuf, &temp.QBufFree, &temp.OBL, &temp.OLL, &temp.OMem, &temp.Events, &temp.Cmd)
		
		if err != nil {
			return nil, err
		}

		ret = append(ret, temp)
	}
	return ret,nil
}