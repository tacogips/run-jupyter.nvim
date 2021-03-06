local M = {}
local config = {
	jupyter = {
		endpoint = "http://localhost:8888",
	},
	output = {
		image_view_cmd = nil,
	},
}

function M.build(user_config)
	config = vim.tbl_deep_extend("force", config, user_config)
end

function M.get()
	return config
end

return M
