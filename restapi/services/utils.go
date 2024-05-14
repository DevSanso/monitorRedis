package services

import (
	"net/http"
	"encoding/json"

	"restapi/constant"
	"restapi/types/core"
	"restapi/types/errs"
)

func readBodyFromRequest(r *http.Request, allocSize int) ([]byte, *core.ApplicationResponse) {
	body := make([]byte, allocSize + constant.DUMMY_BYTE_SIZE)
	n, err := r.Body.Read(body)

	if n > allocSize {
		return nil, &core.ApplicationResponse{
			Response: []byte(""),
			Code:     500,
			Err:      &errs.OOMError{Ip: r.Host, Url: r.URL.String(), LimitSize: allocSize, AllocSize: n},
		}
	}

	if err != nil {
		return nil, &core.ApplicationResponse{
			Response: []byte(""),
			Code:     500,
			Err:      &errs.InternalError{Ip: r.Host, Url : r.URL.String(),  Err : err},
		}
	}

	return body, nil
}

func ObjectToJsonString(obj any) ([]byte, error) {
	return json.Marshal(obj)
}