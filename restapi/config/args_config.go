package config

import (
	"os"
	"encoding/json"
)

type PGConfig struct {
	Ip string `json:"ip"`
	Port int `json:"port"`
	Username string `json:"username"`
	Password string `json:"password"`
	DbName string `json:"dbname"`
}

type ArgsConfig struct {
	SqlitePath string `json:"sqlitePath"`
	PgConfig PGConfig `json:"pgConfig"`

	Log struct {
		Level string `json:"level"`
		Path string `json:"Path"`	
	}`json:"log"`

	ServerIp string `json:"ip"`
	ServerPort int `json:"port"`
}

func ReadConfigPathFromOsArgs() (*ArgsConfig, error) {
	p := os.Args[1]
	configData,readErr := os.ReadFile(p)
	if readErr != nil {
		return nil, readErr
	}

	cfg := new(ArgsConfig)
	jsonErr := json.Unmarshal(configData, cfg)
	if jsonErr != nil {
		return nil, jsonErr
	}

	return cfg, nil
}