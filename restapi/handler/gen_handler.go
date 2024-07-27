package handler

import (
	"context"
	"fmt"
	"net/http"

	"restapi/constant"
	"restapi/global/log"
	"restapi/global/pool"
	"restapi/types/core"
	"restapi/types/errs"
)

type handlerImpl struct{
	mux http.ServeMux
}

type CustomHandlerFunc func(r *http.Request) *core.ApplicationResponse

func wrapperCustomHandler(fun CustomHandlerFunc, path string) http.HandlerFunc{
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

		serv:= r.Context().Value(constant.HTTP_CONTEXT_SERVICE_KEY)
		switch path {
		case "client":
			pool.ClientServicePool.Put(serv)
		case "server":
			pool.ServerServicePool.Put(serv)
		case "db":
			pool.DbServicePool.Put(serv)
		default:
			e := errs.InternalError {Ip : r.Host, Url: r.URL.String(), Err: fmt.Errorf("service pool is memory leak [path:%s]", path)}
			log.Info(e.Error())
		}

	}
}

func NewHandler() http.Handler {
	impl := new(handlerImpl)
	
	impl.mux.HandleFunc("/redis/client", wrapperCustomHandler(clientHandler, "client"))
	impl.mux.HandleFunc("/redis/server", wrapperCustomHandler(serverHandler, "server"))
	impl.mux.HandleFunc("/redis/db", wrapperCustomHandler(dbHandler, "db"))
	return impl
}

func (impl *handlerImpl)allocService(r *http.Request) bool {
	path := r.URL.Path
	ctx := r.Context()
	var temp *http.Request = nil
	switch path {
	case "client":
		temp = r.WithContext(context.WithValue(ctx, constant.HTTP_CONTEXT_SERVICE_KEY, pool.ClientServicePool.Get()))
	case "server":
		temp = r.WithContext(context.WithValue(ctx, constant.HTTP_CONTEXT_SERVICE_KEY, pool.ServerServicePool.Get()))
	case "db":
		temp = r.WithContext(context.WithValue(ctx, constant.HTTP_CONTEXT_SERVICE_KEY, pool.DbServicePool.Get()))
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
