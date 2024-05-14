package repos

import (
	"context"
	"database/sql"
	"sync"

	"restapi/types/errs"
	"restapi/global"
	"restapi/types/repo_vo"
)

type clientRepo struct {
	collectConn *sql.Conn
	initErr error
	closeOnce sync.Once

}

func NewClientRepo(ctx context.Context) *clientRepo {
	conn, connErr := global.GetCollectConn(ctx)
	
	return &clientRepo{
		collectConn: conn,
		initErr: connErr,
	}
}
func (cr *clientRepo)Close() error {
	var err error = nil
	cr.closeOnce.Do(func() {
		if cr.collectConn == nil {return}
		err = cr.Close()
		cr.collectConn = nil
	})
	return err
}
func (cr *clientRepo)List(id int) ([]repo_vo.ClientInfoVO, error) {
	if cr.initErr != nil {
		if cr.collectConn != nil {cr.collectConn.Close()}
		return nil, &errs.ServerDbConnFailedError{Source: cr.initErr, Server: "collect"}
	}

	return nil,nil
}