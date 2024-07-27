package repos

import (
	"context"

	r_query "restapi/constant/query/redis"
	"restapi/dao"
	"restapi/repos/internal"
	"restapi/types/errs"
	"restapi/types/repo_vo"
)

type ServerRepo struct{}

func (cr *ServerRepo) CpuList(id int, min string, max string, ctx context.Context) ([]repo_vo.ServerCpuVO, error) {
	collectDao, daoErr := dao.NewStdDao(ctx, dao.CollectDB)
	if daoErr != nil {
		return nil, &errs.ServerDbConnFailedError{Source: daoErr, Server: "collect"}
	}
	defer collectDao.Close()

	res, QueryErr := dao.StdQueryRun[repo_vo.ServerCpuVO](collectDao, r_query.ClientListQuery, internal.ServerRepoGenInfoCpuList, id, min, max)
	if QueryErr != nil {
		return nil, &errs.ServerDbConnExcuteError{Source: QueryErr, Server: "collect", ObjectNames: []string{"client_list"}}
	}

	return res, nil
}

func (cr *ServerRepo) Stats(id int, collectTime string, ctx context.Context) (*repo_vo.ServerStatVO, error) {
	collectDao, daoErr := dao.NewStdDao(ctx, dao.CollectDB)
	if daoErr != nil {
		return nil, &errs.ServerDbConnFailedError{Source: daoErr, Server: "collect"}
	}
	defer collectDao.Close()

	res, QueryErr := dao.StdQueryOneRun[repo_vo.ServerStatVO](collectDao, r_query.InfoStatsQuery, internal.ServerRepoGenInfoStats, id, collectTime)
	if QueryErr != nil {
		return nil, &errs.ServerDbConnExcuteError{Source: QueryErr, Server: "collect", ObjectNames: []string{"client_list"}}
	}

	return res, nil
}
