package main

import (
	"fmt"
	"net/http"

	"restapi/config"
	"restapi/global"
	"restapi/global/log"
	"restapi/handler"
)
func main() {
	conf, confErr := config.ReadConfigPathFromOsArgs()
	if confErr != nil { panic(confErr) }

	log.InitLog(conf.Log.Level, &conf.Log.Path);

	initDbErr := global.InitDb(conf)
	if initDbErr != nil {panic(initDbErr)}

	addr := fmt.Sprintf("%s:%d", conf.ServerIp, conf.ServerPort)

	http.ListenAndServe(addr, handler.NewHandler())
}