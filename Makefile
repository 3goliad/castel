all:
	pandoc local.org -o README.md && rm local.org
local:
	pandoc README.md -o local.org
