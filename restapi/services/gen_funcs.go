package services

import 	"restapi/repos"

func NewClientServiceFunc() any {
	return &ClientService{
		repo : repos.ClientRepo{},
	}
}

func NewServerServiceFunc() any {
	return &ServerService{
		repo : repos.ServerRepo{},
	}
}