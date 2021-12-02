package = "aoc2021"
version = "scm-0"
rockspec_format = "3.0"
source = {
   url = "git+ssh://git@github.com/khayyamsaleem/advent-of-code.git"
}
description = {
   homepage = "https://github.com/khayyamsaleem/advent-of-code/tree/master/2021/",
   license = "WTFPL"
}
dependencies = {
   "lua >= 5.1";
   "http";
   "inspect";
   "fun";
   "busted";
   "luacov";
   "luafilesystem";
}
build = {
   type = "builtin",
   modules = {}
}
