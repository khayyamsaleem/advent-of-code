local dotenv = {}
local function file_exists(file)
    local f = io.open(file, "rb")
    if f then f:close() end
    return f ~= nil
end

local function lines_from(file)
    if not file_exists(file) then return {} end
    local lines = {}
    for line in io.lines(file) do
        lines[#lines + 1] = line
    end
    return lines
end

function dotenv.load()
    local lines = lines_from(".env")
    local t = {}
    for i, line in pairs(lines) do
        local key, value = string.match(line, "(.+)=(.+)")
        t[key] = value
    end
    return t
end

return dotenv
