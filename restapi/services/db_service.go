package services

import (
	"fmt"
	"net/http"
	"strconv"

	"restapi/repos"
	"restapi/types/core"
	"restapi/types/errs"
)

type DbService struct{repo repos.DbRepo}

func (c *DbService) TopkeyUsage(r *http.Request) *core.ApplicationResponse {
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

	startTime := q.Get("start_time")

	if !isYYYYMMDDHHMI24SSFormat(startTime) {
		return &core.ApplicationResponse{
			Response: []byte(""),
			Code:     400,
			Err:      &errs.BadRequestError{Ip: r.Host, Url: r.URL.String(), Msg: fmt.Sprintf("time format not matching(time:%s)", startTime)},
		}
	}

	endTime := q.Get("end_time")

	if !isYYYYMMDDHHMI24SSFormat(endTime) {
		return &core.ApplicationResponse{
			Response: []byte(""),
			Code:     400,
			Err:      &errs.BadRequestError{Ip: r.Host, Url: r.URL.String(), Msg: fmt.Sprintf("time format not matching(time:%s)", endTime)},
		}
	}

	dbRepo := c.repo

	list, listErr := dbRepo.TopkeyUsage(castId, startTime, endTime, r.Context())

	errRes := writeIfCommonErrorFromAppResponse(listErr, r.Host, r.URL.String())
	if errRes != nil { return errRes }

	return writeCommonResultFromAppResponse(list)
}

func (c *DbService) KeySpaceInfo(r *http.Request) *core.ApplicationResponse {
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

	startTime := q.Get("start_time")

	if !isYYYYMMDDHHMI24SSFormat(startTime) {
		return &core.ApplicationResponse{
			Response: []byte(""),
			Code:     400,
			Err:      &errs.BadRequestError{Ip: r.Host, Url: r.URL.String(), Msg: fmt.Sprintf("time format not matching(time:%s)", startTime)},
		}
	}

	endTime := q.Get("end_time")

	if !isYYYYMMDDHHMI24SSFormat(endTime) {
		return &core.ApplicationResponse{
			Response: []byte(""),
			Code:     400,
			Err:      &errs.BadRequestError{Ip: r.Host, Url: r.URL.String(), Msg: fmt.Sprintf("time format not matching(time:%s)", endTime)},
		}
	}

	dbRepo := c.repo

	list, listErr := dbRepo.KeySpaceInfo(castId, startTime, endTime, r.Context())

	errRes := writeIfCommonErrorFromAppResponse(listErr, r.Host, r.URL.String())
	if errRes != nil { return errRes }

	return writeCommonResultFromAppResponse(list)	
}


