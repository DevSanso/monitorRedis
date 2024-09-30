package redis

const (
	KeyUsageTopTenHundredQuery = `
	SELECT key_name, collect_time, usage_byte, expired_sec FROM redis_key WHERE link_key = ? AND collect_time BETWEEN to_timestamp(?,'YYYYMMDDHHMI24SS') AND to_timestamp(?,'YYYYMMDDHHMI24SS');
	`

	KeySpaceQuery = `
	SELECT dbname, collect_time, keys, expires, avg_ttl FROM redis_keyspace WHERE link_key = ? AND collect_time BETWEEN to_timestamp(?,'YYYYMMDDHHMI24SS') AND to_timestamp(?,'YYYYMMDDHHMI24SS');
	`
)