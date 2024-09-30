package repos

import (
	"context"

	"restapi/types/errs"
	"restapi/types/repo_vo"
	"restapi/dao"
	"restapi/repos/internal"
	r_query "restapi/constant/query/redis"
)

type DbRepo struct {}

func (dr *DbRepo)TopkeyUsage(id int, startTime string, endTime string, ctx context.Context) ([]repo_vo.DbKeyMemUsage, error) {
	collectDao, daoErr := dao.NewStdDao(ctx, dao.CollectDB)
	if daoErr != nil {
		return nil, &errs.ServerDbConnFailedError{Source: daoErr, Server: "collect"}
	}
	defer collectDao.Close()

	res, QueryErr := dao.StdQueryRun[repo_vo.DbKeyMemUsage](collectDao, r_query.ClientListQuery, internal.DbRepoGenKeyTopUsage, id, startTime, endTime)
	if QueryErr != nil {
		return nil, &errs.ServerDbConnExcuteError{Source: QueryErr, Server : "collect", ObjectNames: []string{"client_list"}}
	}

	return res, nil
}

func (dr *DbRepo)KeySpaceInfo(id int, startTime string, endTime string, ctx context.Context) ([]repo_vo.DbKeySpaceInfo, error) {
	collectDao, daoErr := dao.NewStdDao(ctx, dao.CollectDB)
	if daoErr != nil {
		return nil, &errs.ServerDbConnFailedError{Source: daoErr, Server: "collect"}
	}
	defer collectDao.Close()

	res, QueryErr := dao.StdQueryRun[repo_vo.DbKeySpaceInfo](collectDao, r_query.ClientListQuery, internal.DbRepoGenKeySpaceInfo, id, startTime, endTime)
	if QueryErr != nil {
		return nil, &errs.ServerDbConnExcuteError{Source: QueryErr, Server : "collect", ObjectNames: []string{"client_list"}}
	}

	return res, nil
}