package config

type PGConfig struct {
	Ip string `json:"ip"`
	Port int `json:"port"`
	Username string `json:"username"`
	Password string `json:"password"`
	DbName string `json:"dbname"`
}

type ArgsConfig struct {
	SqllitePath string `json:"sqlitePath"`
	PgConfig PGConfig `json:"pgConfig"`
}