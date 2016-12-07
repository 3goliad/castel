#[macro_use] extern crate log;

mod geometry;

use geometry::Dimensions;

enum Backend {
  DRM,
  X11,
}

enum Event {
  Readable,
  Writable,
  Hangup,
  Error,
}

struct PropertyUpdate {
  title: bool,
  class: bool,
  app_id: bool,
  pid: bool,
}

enum ResizeEdge {
  Top,
  Bottom,
  Left,
  TopLeft,
  BottomLeft,
  Right,
  TopRight,
  BottomRight,
}

struct Mods {
  shift: bool,
  caps: bool,
  control: bool,
  alt: bool,
  mod2: bool,
  mod3: bool,
  logo: bool,
  mod5: bool,
}

struct LED {
  num: bool,
  caps: bool,
  scroll: bool,
}

enum Key {
  Released,
  Pressed,
}

enum Button {
  Released,
  Pressed,
}

struct Axis {
  vertical: bool,
  horizontal: bool,
}

enum Touch {
  Down,
  Up,
  Motion,
  Frame,
  Cancel,
}

struct Modifiers {
  leds: LED,
  mods: Mods,
}

struct Handle {
  foo: i32,
}

struct EventSource {
  foo: i32,
}
/* -- Callbacks API */

/* Output was created. Return false if you want to destroy the output. (e.g. failed to allocate data related to view) */
//void wlc_set_output_created_cb(bool (*cb)(wlc_handle output));

/* Output was destroyed. */
//void wlc_set_output_destroyed_cb(void (*cb)(wlc_handle output));

/* Output got or lost focus. */
//void wlc_set_output_focus_cb(void (*cb)(wlc_handle output, bool focus));

/* Output resolution changed. */
//void wlc_set_output_resolution_cb(void (*cb)(wlc_handle output, const struct wlc_size *from, const struct wlc_size *to));

/* Output pre render hook. */
//void wlc_set_output_render_pre_cb(void (*cb)(wlc_handle output));

/* Output post render hook. */
//void wlc_set_output_render_post_cb(void (*cb)(wlc_handle output));

/* Output context is created. This generally happens on startup and when current tty changes */
//void wlc_set_output_context_created_cb(void (*cb)(wlc_handle output));

/* Output context was destroyed. */
//void wlc_set_output_context_destroyed_cb(void (*cb)(wlc_handle output));

/* View was created. Return false if you want to destroy the view. (e.g. failed to allocate data related to view) */
//void wlc_set_view_created_cb(bool (*cb)(wlc_handle view));

/* View was destroyed. */
//void wlc_set_view_destroyed_cb(void (*cb)(wlc_handle view));

/* View got or lost focus. */
//void wlc_set_view_focus_cb(void (*cb)(wlc_handle view, bool focus));

/* View was moved to output. */
//void wlc_set_view_move_to_output_cb(void (*cb)(wlc_handle view, wlc_handle from_output, wlc_handle to_output));

/* Request to set given geometry for view. Apply using wlc_view_set_geometry to agree. */
//void wlc_set_view_request_geometry_cb(void (*cb)(wlc_handle view, const struct wlc_geometry*));

/* Request to disable or enable the given state for view. Apply using wlc_view_set_state to agree. */
//void wlc_set_view_request_state_cb(void (*cb)(wlc_handle view, enum wlc_view_state_bit, bool toggle));

/* Request to move itself. Start a interactive move to agree. */
//void wlc_set_view_request_move_cb(void (*cb)(wlc_handle view, const struct wlc_point*));

/* Request to resize itself with the given edges. Start a interactive resize to agree. */
//void wlc_set_view_request_resize_cb(void (*cb)(wlc_handle view, uint32_t edges, const struct wlc_point*));

/* View pre render hook. */
//void wlc_set_view_render_pre_cb(void (*cb)(wlc_handle view));

/* View post render hook. */
//void wlc_set_view_render_post_cb(void (*cb)(wlc_handle view));

/* View properties (title, class, app_id) was updated */
//void wlc_set_view_properties_updated_cb(void (*cb)(wlc_handle view, uint32_t mask));

/* Key event was triggered, view handle will be zero if there was no focus. Return true to prevent sending the event to clients. */
//void wlc_set_keyboard_key_cb(bool (*cb)(wlc_handle view, uint32_t time, const struct wlc_modifiers*, uint32_t key, enum wlc_key_state));

/* Button event was triggered, view handle will be zero if there was no focus. Return true to prevent sending the event to clients. */
//void wlc_set_pointer_button_cb(bool (*cb)(wlc_handle view, uint32_t time, const struct wlc_modifiers*, uint32_t button, enum wlc_button_state, const struct wlc_point*));

/* Scroll event was triggered, view handle will be zero if there was no focus. Return true to prevent sending the event to clients. */
//void wlc_set_pointer_scroll_cb(bool (*cb)(wlc_handle view, uint32_t time, const struct wlc_modifiers*, uint8_t axis_bits, double amount[2]));

/* Motion event was triggered, view handle will be zero if there was no focus. Apply with wlc_pointer_set_position to agree. Return true to prevent sending the event to clients. */
//void wlc_set_pointer_motion_cb(bool (*cb)(wlc_handle view, uint32_t time, const struct wlc_point*));

/* Touch event was triggered, view handle will be zero if there was no focus. Return true to prevent sending the event to clients. */
//void wlc_set_touch_cb(bool (*cb)(wlc_handle view, uint32_t time, const struct wlc_modifiers*, enum wlc_touch_type, int32_t slot, const struct wlc_point*));

/* Compositor is ready to accept clients. */
//void wlc_set_compositor_ready_cb(void (*cb)(void));

/* Compositor is about to terminate */
//void wlc_set_compositor_terminate_cb(void (*cb)(void));

/* Input device was created. Return value does nothing. (Experimental) */
//void wlc_set_input_created_cb(bool (*cb)(struct libinput_device *device));

/* Input device was destroyed. (Experimental) */
//void wlc_set_input_destroyed_cb(void (*cb)(struct libinput_device *device));

fn init() -> Result<(), &'static str> {
  unimplemented!();
}

fn terminate() {
  unimplemented!();
}

fn get_backend_type() -> Backend {
  unimplemented!();
}

fn launch(bin: &str, args: &[&str]) {
  unimplemented!();
}

fn run() {
  unimplemented!();
}

fn handle_set_user_data(handle: Handle, userdata: &[u8]) {
  unimplemented!();
}

fn handle_get_user_data(handle: Handle, userdata: &[u8]) {
  unimplemented!();
}

/* Add fd to event loop. Return value of callback is unused, you should return 0. */
//WLC_NONULLV(3) struct wlc_event_source* wlc_event_loop_add_fd(int fd, uint32_t mask, int (*cb)(int fd, uint32_t mask, void *userdata), void *userdata);

/* Add timer to event loop. Return value of callback is unused, you should return 0. */
//WLC_NONULLV(1) struct wlc_event_source* wlc_event_loop_add_timer(int (*cb)(void *userdata), void *userdata);

fn event_source_timer_update(source: EventSource, ms_delay: i32) -> Result<(), &'static str> {
  unimplemented!();
}

fn event_source_remove(source: EventSource) {
  unimplemented!();
}

// Output API

fn get_outputs(out_memb: i32) -> Vec<Output> {
  unimplemented!();
}

fn focus_output(output: Option<Output>) {
  unimplemented!();
}

fn get_focused_output() -> Output {
  unimplemented!();
}
struct Output {
  foo: i32,
  pub name: String,
  sleep: bool,
  resolution: Dimensions,
  virtual_resolution: Dimensions,
  scale: u32,
}

impl Output {
  fn sleep() {
    unimplemented!();
  }

  fn wake() {
    unimplemented!();
  }

  fn is_sleeping(&self) -> bool {
    self.sleep
  }

  fn get_resolution(&self) -> &Dimensions {
    &self.resolution
  }

  fn get_virtual_resolution(&self) -> &Dimensions {
    &self.virtual_resolution
  }

  fn set_resolution(&mut self, res: Dimensions, scale: u32) {
    unimplemented!();
  }

  fn get_scale(&self) -> u32 {
    self.scale
  }

  fn get_visibility(&self) {
    unimplemented!();
  }

  fn get_views(out_memb: i32) -> Vec<View> {
    unimplemented!();
  }

  fn get_mut_views(out_memb: i32) -> Vec<View> {
    unimplemented!();
  }

  fn set_views(views: Vec<View>) -> Result<(), &'static str> {
    unimplemented!();
  }

}

struct View {
  foo: i32,
  override_redirect: bool,
  unmanaged: bool,
  splash: bool,
  modal: bool,
  popup: bool,
  maximized: bool,
  fullscreen: bool,
  resizing: bool,
  moving: bool,
  activated: bool,
}

impl View {
  //TODO property setters/getters
  fn focus() {
    unimplemented!();
  }

  fn close() {
    unimplemented!();
  }

  fn get_output() -> Output {
    unimplemented!();
  }

  fn set_output() {
    unimplemented!()
  }

  fn send_to_back() {
    unimplemented!()
  }

  fn send_below() {
    unimplemented!()
  }

  fn bring_above() {
    unimplemented!()
  }

  fn get_visibility() {
    unimplemented!()
  }

  fn set_visibility() {
    unimplemented!()
  }

  fn get_geometry() {
    unimplemented!();
  }

  fn get_visible_geometry() {
    unimplemented!();
  }

  fn set_geometry() {
    unimplemented!();
  }

  fn get_parent() {
    unimplemented!();
  }

  fn set_parent() {
    unimplemented!();
  }

  fn get_class() {
    unimplemented!();
  }

  fn get_app_id() {
    unimplemented!();
  }

  fn get_title() {
    unimplemented!();
  }

  fn get_pid() {
    unimplemented!();
  }
}

fn get_pointer_position() {
  unimplemented!();
}

fn set_pointer_position() {
  unimplemented!();
}
