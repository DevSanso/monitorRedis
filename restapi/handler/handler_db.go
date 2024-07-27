package handler

import (
	"fmt"
	"net/http"
	"restapi/services"
	"restapi/types/core"
	"restapi/types/errs"
	"restapi/constant"
)

func dbHandler(r *http.Request) *core.ApplicationResponse {
	query := r.URL.Query()
	action := query.Get("action")
	
	var service *services.DbService = r.Context().Value(constant.HTTP_CONTEXT_SERVICE_KEY).(*services.DbService) 
	var res *core.ApplicationResponse = nil

	switch action {
	case "keyTopUsage":
		res = service.TopkeyUsage(r)
	default:
		res = &core.ApplicationResponse{
			Response: []byte("bad request"),
			Err: &errs.BadRequestError{Ip : r.Host, Url: r.URL.String(), Msg: fmt.Sprintf("unsupport action : %s", action)},
			Code: 400,
		}
	}

	return res
}