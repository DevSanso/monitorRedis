package repos

import (
	"context"

	"restapi/types/errs"
	"restapi/types/repo_vo"
	"restapi/dao"
	"restapi/repos/internal"
	r_query "restapi/constant/query/redis"
)

type ClientRepo struct {}

func (cr *ClientRepo)List(id int, ctx context.Context) ([]repo_vo.ClientInfoVO, error) {
	collectDao, daoErr := dao.NewStdDao(ctx, dao.CollectDB)
	if daoErr != nil {
		return nil, &errs.ServerDbConnFailedError{Source: daoErr, Server: "collect"}
	}
	defer collectDao.Close()

	res, QueryErr := dao.StdQueryRun[repo_vo.ClientInfoVO](collectDao, r_query.ClientListQuery, internal.ClientRepoGenClientList, id)
	if QueryErr != nil {
		return nil, &errs.ServerDbConnExcuteError{Source: QueryErr, Server : "collect", ObjectNames: []string{"client_list"}}
	}

	return res, nil
}