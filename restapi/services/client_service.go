package services

import (
	"fmt"
	"net/http"
	"strconv"

	"restapi/repos"
	"restapi/types/core"
	"restapi/types/errs"
	"restapi/utils"
)

type ClientService struct{}

func (c *ClientService) List(r *http.Request) *core.ApplicationResponse {
	q := r.URL.Query()
	id := q.Get("object_id")
	castId, castErr := strconv.Atoi(id)

	if castErr != nil {
		return &core.ApplicationResponse{
			Response: []byte(""),
			Code:     400,
			Err:      &errs.BadRequestError{Ip: r.Host, Url: r.URL.String(), Msg: fmt.Sprintf("object_id(%s) can't cast", id)},
		}
	}
	
	clientRepo := repos.NewClientRepo(r.Context())
	defer clientRepo.Close()

	list, listErr := clientRepo.List(castId)

	if listErr != nil {
		ret := new(core.ApplicationResponse)
		ret.Err = &errs.InternalError{
			Ip:  r.Host,
			Url: r.URL.String(),
			Err: listErr,
		}

		if utils.CheckErrorIs[*errs.ServerDbConnFailedError](listErr) {
			ret.Code = 500
		}
		if utils.CheckErrorIs[*errs.NoDataError](listErr) {
			ret.Response = []byte("[]")
			ret.Code = 204
		}
		
		return ret
	}

	ret := new(core.ApplicationResponse)

	body, jsonErr := ObjectToJsonString(list)
	ret.Response = body
	ret.Err = jsonErr

	if jsonErr != nil {
		ret.Code = 500
	} else {
		ret.Code = 200
	}
	return ret
}
