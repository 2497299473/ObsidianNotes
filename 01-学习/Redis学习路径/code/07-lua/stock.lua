-- Redis 阶段三 · Lua 脚本
-- 对应笔记: [[01-事务与Lua脚本]]
-- 原子库存扣减：库存充足则扣减，否则拒绝

local stock = tonumber(redis.call('GET', KEYS[1]) or '0')
local delta = tonumber(ARGV[1])
if stock >= delta then
    redis.call('DECRBY', KEYS[1], delta)
    return 1
else
    return 0
end
