local lib = {}

function lib.iter_hack(riter)
  return function() local v = riter() ; return v, v ; end
end

return lib