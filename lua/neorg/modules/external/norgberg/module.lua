require("neorg.modules.base")

local module = neorg.modules.create("external.norgberg")

module.load = function ()
  print(vim.inspect(require('norgberg').greet_people({ 'from Norgberg module running rust' })))
end

return module
