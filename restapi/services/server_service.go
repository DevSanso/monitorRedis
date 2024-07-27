package services

import (
	"fmt"
	"net/http"
	"strconv"

	"restapi/repos"
	"restapi/types/core"
	"restapi/types/errs"
	"restapi/types/service_vo"
)

type ServerService struct{ repo repos.ServerRepo }

func (c *ServerService) CpuList(r *http.Request) *core.ApplicationResponse {
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

	serverRepo := c.repo

	list, listErr := serverRepo.CpuList(castId, min, max, r.Context())

	errRes := writeIfCommonErrorFromAppResponse(listErr, r.Host, r.URL.String())
	if errRes != nil {
		return errRes
	}

	return writeCommonResultFromAppResponse(service_vo.NewServerCpuUsageVO(list))
}

func (c *ServerService) Stats(r *http.Request) *core.ApplicationResponse {
	q := r.URL.Query()
	collectTime := q.Get("collect_time")
	id := q.Get("object_id")
	castId, castErr := strconv.Atoi(id)

	if !isYYYYMMDDHHMI24SSFormat(collectTime) {
		return &core.ApplicationResponse{
			Response: []byte(""),
			Code:     400,
			Err:      &errs.BadRequestError{Ip: r.Host, Url: r.URL.String(), Msg: fmt.Sprintf("time format not matching(time:%s)", collectTime)},
		}
	}

	if castErr != nil {
		return &core.ApplicationResponse{
			Response: []byte(""),
			Code:     400,
			Err:      &errs.BadRequestError{Ip: r.Host, Url: r.URL.String(), Msg: fmt.Sprintf("object_id(%s) can't cast", id)},
		}
	}

	serverRepo := c.repo

	stat, statErr := serverRepo.Stats(castId, collectTime, r.Context())

	errRes := writeIfCommonErrorFromAppResponse(statErr, r.Host, r.URL.String())
	if errRes != nil {
		return errRes
	}

	return writeCommonResultFromAppResponse(stat)
}
