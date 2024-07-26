package pool

import (
	"sync"

	"restapi/services"
	"restapi/types/errs"
)

type ServicePool struct {
	p sync.Pool
	alloc int
	used int
	max int
	m sync.Mutex
}

func newServicePool(name string, count int, gen func() any) *ServicePool {
	obj := &ServicePool{
		p : sync.Pool{},
		alloc: 0,
		max :count,
		m : sync.Mutex{},
	}
	
	obj.p.New = func() any {
		if obj.alloc >= obj.max {
			return errs.OutRangeIdxError{Ip: "localhost", Method: name, Limit: obj.max}
		}

		obj.alloc += 1
		obj.used += 1
		ret := gen()
		return ret
	}

	return obj
}

func (s *ServicePool)Get() any {
	s.m.Lock()
	var item = s.p.Get()
	s.used += 1
	s.m.Unlock()
	return item
}

func (s *ServicePool)Put(item any) {
	s.m.Lock()
	s.used -= 1
	s.p.Put(item)
	s.m.Unlock()
}

var (
	ServerServicePool = newServicePool("serverServicePool", 100, services.NewServerServiceFunc)
	ClientServicePool = newServicePool("serverServicePool", 100, services.NewClientServiceFunc)
)

