package services

import (
	"fmt"
	"net/http"
	"strconv"

	"restapi/repos"
	"restapi/types/core"
	"restapi/types/errs"
)

type InfoService struct{}

func (c *InfoService) CpuList(r *http.Request) *core.ApplicationResponse {
	q := r.URL.Query()
	min := q.Get("min_time")
	max := q.Get("max_time")
	id := q.Get("object_id")
	castId, castErr := strconv.Atoi(id)

	if !isYYYYMMDDHHMI24SSFormat(min) || !isYYYYMMDDHHMI24SSFormat(max) {
		return &core.ApplicationResponse{
			Response: []byte(""),
			Code:     400,
			Err:      &errs.BadRequestError{Ip: r.Host, Url: r.URL.String(), Msg: fmt.Sprintf("time format not matching(min:%s,max%s)", min, max)},
		}
	}

	if castErr != nil {
		return &core.ApplicationResponse{
			Response: []byte(""),
			Code:     400,
			Err:      &errs.BadRequestError{Ip: r.Host, Url: r.URL.String(), Msg: fmt.Sprintf("object_id(%s) can't cast", id)},
		}
	}

	infoRepo := repos.NewInfoRepo(r.Context())

	list, listErr := infoRepo.CpuList(castId, min, max)

	errRes := writeIfCommonErrorFromAppResponse(listErr, r.Host, r.URL.String())
	if errRes != nil { return errRes }

	return writeCommonResultFromAppResponse(list)
}
