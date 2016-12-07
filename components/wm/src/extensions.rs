// let desktop_shell = DesktopShellState::new();

fn find_or_create_panel_config(resource: WLResource) -> PanelConfig {
  for panel in desktop_shell.panels {
    let conf = PanelConfig::from(panel);
    if conf.wl_resource == resource {
      debug!("Found existing panel config for resource {:?}", resource);
      return conf;
    }
  }
  debug!("Creating panel config for resource {:?}", resource);
  let conf = PanelConfig::new();
  desktop_shell.panes.add(conf);
  conf.resource = resource;
  return conf;
}

fn set_background(client: WLClient, resource: WLResource, output: WLResource, surface: WLResource) {
  let output = wlc_handle_from_wl_output_resource(output)?;
  debug!("Setting surface {:?} as background for output {:?}", surface, output);
  let config = BackgroundConfig::new();
  config.client = client;
  config.output = output;
  config.surface = wlc_resource_from_wl_surface_resource(surface);
  config.wl_surface_res = surface;
  desktop_shell.backgrounds.add(config);
	arrange_windows(swayc_by_handle(output), -1, -1);
	wlc_output_schedule_render(config.output);
}

fn set_panel(client: WLClient, resource: WLResource, output: WLResource, surface: WLResource) {
  let output = wlc_handle_from_wl_output_resource(output)?;
  debug!("Setting surface {:?} as panel for output {:?} (wl_resource:{:?}", surface, output, resource);
  let config = find_or_create_panel_config();
  config.output = output;
  config.client = client;
  config.surface = wlc_resource_from_wl_surface_resource(surface);
  config.wl_surface_res = surface;
	arrange_windows(&root_container, -1, -1);
	wlc_output_schedule_render(config.output);
}

fn set_panel_position(client: WLClient, resource: WLResource, position: u32) {
  let config = find_or_create_panel_config(resource);
  debug!("Panel position for wl_resource {:?} changed {:?} => {:?}", resource, config.panel_position, position);
  config.panel_position = position;
  arrange_windows(&root_container, -1, 1);
}

//static struct desktop_shell_interface desktop_shell_implementation = {
//	.set_background = set_background,
//	.set_panel = set_panel,
//	.set_lock_surface = desktop_set_lock_surface,
//	.unlock = desktop_unlock,
//	.set_grab_surface = set_grab_surface,
//	.desktop_ready = desktop_ready,
//	.set_panel_position = set_panel_position
//};

fn desktop_shell_bind(client: WLClient, version:u8, id:u8) {
  if version > 3 {
    return;
  }

  let resource = wl_resource_create(client, desktop_shell_interface, version, id);
  if resource.is_err() {
    wl_client_post_no_memory(client);
  }

  wl_resource_set_implementation(resource, desktop_shell_implementation, NULL, NULL);
}

pub fn register_extensions() {
  wl_global_create(wlc_get_wl_display(), desktop_shell_interface, 3, NULL, desktop_shell_bind);
  desktop_shell.backgrounds = Vec::new();
  desktop_shell.panels = Vec::new();
}
