package services

import (
	"encoding/json"
	"net/http"
	"strconv"

	"restapi/constant"
	"restapi/types/core"
	"restapi/types/errs"
	"restapi/utils"
)

func readBodyFromRequest(r *http.Request, allocSize int) ([]byte, *core.ApplicationResponse) {
	body := make([]byte, allocSize+constant.DUMMY_BYTE_SIZE)
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
			Err:      &errs.InternalError{Ip: r.Host, Url: r.URL.String(), Err: err},
		}
	}

	return body, nil
}

func writeIfCommonErrorFromAppResponse(err error, host string, url string) *core.ApplicationResponse {
	if err == nil {
		return nil
	}
	ret := new(core.ApplicationResponse)

	ret.Err = &errs.InternalError{
		Ip:  host,
		Url: url,
		Err: err,
	}

	if utils.CheckErrorIs[*errs.ServerDbConnFailedError](err) {
		ret.Code = 500
	}
	if utils.CheckErrorIs[*errs.NoDataError](err) {
		ret.Response = []byte("[]")
		ret.Code = 204
	}

	return ret
}

func writeCommonResultFromAppResponse(body any) *core.ApplicationResponse {
	ret := new(core.ApplicationResponse)

	body_bytes, jsonErr := objectToJsonString(body)
	ret.Response = body_bytes
	
	if jsonErr != nil {
		ret.Code = 500
		ret.Err = jsonErr
	} else {
		ret.Code = 200
		ret.Err = nil
	}

	return ret
}

func objectToJsonString(obj any) ([]byte, error) {
	return json.Marshal(obj)
}

func isYYYYMMDDHHMI24SSFormat(data string) bool {
	_, err := strconv.Atoi(data)
	if err != nil && len(data) != 14 {
		return false
	}
	return true
}