package repos

import (
	"context"

	"restapi/types/errs"
	"restapi/types/repo_vo"
	"restapi/dao"
	"restapi/repos/internal"
	r_query "restapi/constant/query/redis"
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