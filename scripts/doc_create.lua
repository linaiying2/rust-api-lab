-- doc_create.lua
local cjson = require("cjson")

-- 构造随机 Markdown 内容（模拟不同长度的文档）
local function random_markdown()
    local lengths = {100, 1000, 10000} -- 100B/1KB/10KB 内容
    local content = "# Random Markdown\n\n"
    local random_len = lengths[math.random(#lengths)]
    -- 填充随机字符（模拟真实内容）
    for i = 1, random_len do
        content = content .. string.char(math.random(97, 122))
    end
    return content
end

function request()
    local path = "/api/documents"
    local headers = {["Content-Type"] = "application/json"}
    local body = cjson.encode({
        content = random_markdown()
    })
    return wrk.format("POST", path, headers, body)
end