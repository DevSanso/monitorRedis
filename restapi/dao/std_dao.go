package dao

import (
	"context"
	"database/sql"
	"sync"

	"restapi/global"
)

type StdDao struct {
	conn *sql.Conn
	ctx context.Context
	o *sync.Once
}

const (
	CollectDB = 0
	InfoDB = 1
)

type DBType int

func NewStdDao(ctx context.Context, selectDb DBType) (*StdDao, error) {
	var c *sql.Conn = nil
	var e error = nil
	if selectDb == CollectDB {
		c, e = global.GetCollectConn(ctx)
	}else {
		c, e = global.GetInfoConn(ctx)
	}

	if e != nil {
		return nil, e
	}

	return &StdDao{
		conn : c,
		o : new(sync.Once),
	}, nil
}

func (sd *StdDao)Close() error {
	var ret error = nil
	sd.o.Do(func() {
		if sd.conn != nil {
			sd.conn.Close()
			sd.conn = nil
			sd.ctx = nil
		}
	})
	return ret
}

func StdQueryRun[T any](sd *StdDao, query string, gen func(*sql.Rows) ([]T,error), args ...any) ([]T,error) {
	ctx := sd.ctx
	rows, err := sd.conn.QueryContext(ctx, query, args)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	ret, retErr := gen(rows)
	return ret, retErr
}

func StdQueryOneRun[T any](sd *StdDao, query string, gen func(*sql.Row) (*T,error), args ...any) (*T,error) {
	ctx := sd.ctx
	row := sd.conn.QueryRowContext(ctx, query, args)
	if row.Err() != nil {
		return nil, nil
	}
	
	ret, retErr := gen(row)
	return ret, retErr
}

