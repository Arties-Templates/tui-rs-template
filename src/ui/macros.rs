macro_rules! render {
  ($renderer:ident, $widget:expr, $chunk:expr) => {
      $renderer.render_widget($widget, $chunk)
  };
}

macro_rules! render_stateful {
  ($renderer:ident, $widget:expr, $chunk:expr, $state:expr) => {
      $renderer.render_stateful_widget($widget, $chunk, $state)
  };
}

pub(crate) use render;
pub(crate) use render_stateful;