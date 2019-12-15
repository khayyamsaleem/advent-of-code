package = "07"
version = "dev-1"
rockspec_format = "3.0"
source = {
   url = "git+ssh://git@github.com/khayyamsaleem/advent-of-code.git"
}
description = {
   homepage = "https://github.com/khayyamsaleem/advent-of-code/tree/master/2019/07/",
   license = "WTFPL"
}
dependencies = {
    "http >= 0.3-0",
    "rapidjson >= 0.6.1-1",
    "inspect >= 3.1.1-0",
    "ham/luacombine >= scm-6"
}
build = {
   type = "builtin",
   modules = {}
}
