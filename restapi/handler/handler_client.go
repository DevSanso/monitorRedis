package handler

import (
	"fmt"
	"net/http"
	"restapi/services"
	"restapi/types/core"
	"restapi/types/errs"
	"restapi/constant"
)

func clientHandler(r *http.Request) *core.ApplicationResponse {
	query := r.URL.Query()
	action := query.Get("action")
	
	var service *services.ClientService = r.Context().Value(constant.HTTP_CONTEXT_SERVICE_KEY).(*services.ClientService) 
	var res *core.ApplicationResponse = nil

	switch action {
	case "list":
		res = service.List(r)
	default:
		res = &core.ApplicationResponse{
			Response: []byte("bad request"),
			Err: &errs.BadRequestError{Ip : r.Host, Url: r.URL.String(), Msg: fmt.Sprintf("unsupport action : %s", action)},
			Code: 400,
		}
	}

	return res
}