package services

import (
	"fmt"
	"net/http"
	"strconv"

	"restapi/repos"
	"restapi/types/core"
	"restapi/types/errs"
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

	list, listErr := clientRepo.List(castId)

	errRes := writeIfCommonErrorFromAppResponse(listErr, r.Host, r.URL.String())
	if errRes != nil { return errRes }

	return writeCommonResultFromAppResponse(list)
}
