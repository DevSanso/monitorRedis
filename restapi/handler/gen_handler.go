package handler

import (
	"context"
	"net/http"

	"restapi/constant"
	"restapi/global/log"
	"restapi/services"
	"restapi/types/core"
	"restapi/types/errs"
)

type handlerImpl struct{
	mux http.ServeMux
}

type CustomHandlerFunc func(r *http.Request) *core.ApplicationResponse

func wrapperCustomHandler(fun CustomHandlerFunc) http.HandlerFunc{
	return func(w http.ResponseWriter, r *http.Request) {
		res := fun(r)

		if res.Response == nil {
			w.Write([]byte(""))
		}else {
			w.Write(res.Response)
		}

		if res.Err != nil {
			if res.Code < 500 {
				log.Info(res.Err.Error())
			}else {
				log.Err(res.Err.Error())
			}
		}
		w.WriteHeader(res.Code)
	}
}

func NewHandler() http.Handler {
	impl := new(handlerImpl)
	
	impl.mux.HandleFunc("/redis/client", wrapperCustomHandler(clientHandler))
	impl.mux.HandleFunc("/redis/info", wrapperCustomHandler(infoHandler))
	return impl
}

func (impl *handlerImpl)allocService(r *http.Request) bool {
	path := r.URL.Path
	ctx := r.Context()
	var temp *http.Request = nil
	switch path {
	case "client":
		temp = r.WithContext(context.WithValue(ctx, constant.HTTP_CONTEXT_SERVICE_KEY, &services.ClientService{}))
	case "info":
		temp = r.WithContext(context.WithValue(ctx, constant.HTTP_CONTEXT_SERVICE_KEY, &services.InfoService{}))
	default:
		e := errs.BadRequestError {Ip : r.Host, Url: r.URL.String(), Msg: "not support AllocService"}
		log.Info(e.Error())
		return false
	}

	*r = *temp
	return true
}

func (impl *handlerImpl)ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if !impl.allocService(r) {
		w.WriteHeader(http.StatusBadRequest)
		return
	}
	
	impl.mux.ServeHTTP(w, r)
	
}
