use crate::theme::ColorStyle;
use crate::view::{View, ViewWrapper};
use crate::Printer;

/// Wrapper view that fills the background.
///
/// This is mostly used as layer in the [`StackView`].
///
/// [`StackView`]: struct.StackView.html
#[derive(Debug)]
pub struct Layer<T: View> {
    view: T,
    color: ColorStyle,
}

impl<T: View> Layer<T> {
    /// Wraps the given view.
    pub fn new(view: T) -> Self {
        Self::with_color(view, ColorStyle::primary())
    }

    /// Wraps the given view with a custom background color.
    pub fn with_color(view: T, color: ColorStyle) -> Self {
        Layer { view, color }
    }

    /// Gets the current color.
    pub fn color(&self) -> ColorStyle {
        self.color
    }

    /// Sets the background color.
    pub fn set_color(&mut self, color: ColorStyle) {
        self.color = color;
    }

    inner_getters!(self.view: T);
}

impl<T: View> ViewWrapper for Layer<T> {
    wrap_impl!(self.view: T);

    fn wrap_draw(&self, printer: &Printer<'_, '_>) {
        printer.with_color(self.color, |printer| {
            for y in 0..printer.size.y {
                printer.print_hline((0, y), printer.size.x, " ");
            }
        });
        self.view.draw(printer);
    }
}
