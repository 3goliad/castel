fn init_layout() {
  root_container.id = 0;
  root_container.type = C_ROOT;
  root_container.layout = L_NONE;
	root_container.name = "root";
	root_container.children = Vec::new();
	root_container.handle = -1;
	root_container.visible = true;
	current_focus = &root_container;
	scratchpad = Vec::new();
}
