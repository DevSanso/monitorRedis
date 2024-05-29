package global

import (
	"context"
	"database/sql"
	"fmt"
	"path"
	"sync"
	"time"

	_ "github.com/lib/pq"
	_ "github.com/mattn/go-sqlite3"

	"restapi/config"
)

var (
	onceInit sync.Once

	infoDb *sql.DB
	collectDb *sql.DB
)

func GetInfoConn(ctx context.Context) (*sql.Conn, error) {
	return infoDb.Conn(ctx)
}

func GetCollectConn(ctx context.Context) (*sql.Conn, error) {
	return collectDb.Conn(ctx)
}

func InitDb(config *config.ArgsConfig) error {
	var ret error = nil

	onceInit.Do(func() {
		sqlPath := path.Join(config.SqlitePath)

		info,infoErr := sql.Open("sqlite",fmt.Sprintf("file:%s?cache=shared&mode=ro", sqlPath));
		if infoErr != nil {
			ret = infoErr
			return
		}
		pgConfig := config.PgConfig
		pgUrl := fmt.Sprintf("postgres://%s:%s@%s:%d/%s?sslmode=verify-full", pgConfig.Username,
		  pgConfig.Password,
		  pgConfig.Ip, 
		  pgConfig.Port, 
		  pgConfig.DbName)
	
		collect, collectErr := sql.Open("postgres", pgUrl)
		if collectErr != nil {
			info.Close()
			ret = collectErr
			return
		}
		info .SetMaxIdleConns(2)
		collect.SetMaxIdleConns(5)
	
		info.SetConnMaxIdleTime(time.Second * 30)
		collect.SetConnMaxIdleTime(time.Second * 40)
	
		infoDb = info
		collectDb = collect
	});

	return ret
}