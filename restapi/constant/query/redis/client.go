package redis

const (
	ClientListQuery = `
	SELECT  
		collect_time, id, addr, fd, name, age, idle, flags, db, sub, psub, multi, qbuf, qbuf_free, obl, oll, omem, events, cmd 
	FROM client_list WHERE unqiue_id  = ? AND collect_time BETWEEN to_timestamp(?,'YYYYMMDDHHMI24SS') AND to_timestamp(?,'YYYYMMDDHHMI24SS');
	` 
)