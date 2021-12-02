require "set_paths"
local lfs = require "lfs";

local day = arg[1]

if day == nil then
  print("Must supply argument for \"day\'")
  os.exit(1)
end
if string.match(day, "[012]%d") == nil then
  print("Invalid argument for day, must be a 2 digit number (leading 0 if < 10)")
  os.exit(1)
end
if tonumber(day) > 25 then
  print("Invalid argument for day, must be ≤ 25")
  os.exit(1)
end

for d in lfs.dir(".") do
  if day == d then
    require(day .. "/run")
    os.exit(0)
  end
end

print("That day has not been completed yet. Sorry!")
os.exit(1)
