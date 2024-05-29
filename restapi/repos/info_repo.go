package repos

import (
	"context"

	r_query "restapi/constant/query/redis"
	"restapi/dao"
	"restapi/repos/internal"
	"restapi/types/errs"
	"restapi/types/repo_vo"
)

type infoRepo struct {ctx context.Context}

func NewInfoRepo(ctx context.Context) *infoRepo {
	return &infoRepo{ctx :ctx}
}

func (cr *infoRepo)CpuList(id int, min string, max string) ([]repo_vo.InfoCpuVO, error) {
	collectDao, daoErr := dao.NewStdDao(cr.ctx, dao.CollectDB)
	if daoErr != nil {
		return nil, &errs.ServerDbConnFailedError{Source: daoErr, Server: "collect"}
	}
	defer collectDao.Close()

	res, QueryErr := dao.StdQueryRun[repo_vo.InfoCpuVO](collectDao, r_query.ClientListQuery, internal.InfoRepoGenInfoCpuList,id, min, max)
	if QueryErr != nil {
		return nil, &errs.ServerDbConnExcuteError{Source: QueryErr, Server : "collect", ObjectNames: []string{"client_list"}}
	}

	return res, nil
}

func (cr *infoRepo)Stats(id int, collectTime string) (*repo_vo.InfoStatVO, error) {
	collectDao, daoErr := dao.NewStdDao(cr.ctx, dao.CollectDB)
	if daoErr != nil {
		return nil, &errs.ServerDbConnFailedError{Source: daoErr, Server: "collect"}
	}
	defer collectDao.Close()

	res, QueryErr := dao.StdQueryOneRun[repo_vo.InfoStatVO](collectDao, r_query.InfoStatsQuery, internal.InfoRepoGenInfoStats,id, collectTime)
	if QueryErr != nil {
		return nil, &errs.ServerDbConnExcuteError{Source: QueryErr, Server : "collect", ObjectNames: []string{"client_list"}}
	}

	return res, nil
}