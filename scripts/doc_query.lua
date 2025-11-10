-- doc_query.lua
local cjson = require("cjson")

-- 初始化：设置请求头（适配 Cloud Sync 的认证，可选）
function setup(thread)
    thread:set("auth_header", "Authorization: Bearer test_token")
    thread:set("doc_ids", {1, 2, 3, 4, 5}) -- 提前准备的 5 个大文档 ID
end

-- 构造请求：循环请求不同文档 ID
function request()
    local doc_ids = wrk.thread:get("doc_ids")
    local random_id = doc_ids[math.random(#doc_ids)]
    local path = "/api/documents/" .. random_id .. "/zero-copy" -- 零拷贝接口
    local headers = {
        ["Content-Type"] = "application/json",
        [wrk.thread:get("auth_header"):match("([^:]+)")] = wrk.thread:get("auth_header"):match(": (.+)")
    }
    return wrk.format("GET", path, headers)
end

-- 响应处理：统计成功/失败率
function response(status, headers, body)
    if status ~= 200 then
        wrk.log("Query failed: status=" .. status .. ", body=" .. body)
    end
end