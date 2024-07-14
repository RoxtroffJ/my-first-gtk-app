use gtk::gio::Settings;
use gtk::{Application, AccessibleRole, Align, LayoutManager, Overflow, Widget, Window};
use gtk::glib;
use glib::prelude::*;

/// A [builder-pattern] type to construct [`SavingWindow`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SavingWindowBuilder {
    builder: glib::object::ObjectBuilder<'static, super::SavingWindow>,
}

impl SavingWindowBuilder {
    pub(super) fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    /// The [`Application`][crate::Application] associated with the window.
    ///
    /// The application will be kept alive for at least as long as it
    /// has any windows associated with it (see g_application_hold()
    /// for a way to keep it alive without windows).
    ///
    /// Normally, the connection between the application and the window
    /// will remain until the window is destroyed, but you can explicitly
    /// remove it by setting the :application property to [`None`].
    pub fn application(self, application: &impl IsA<Application>) -> Self {
        Self {
            builder: self
                .builder
                .property("application", application.clone().upcast()),
        }
    }

    /// The child widget.
    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    /// Whether the window should have a frame (also known as *decorations*).
    pub fn decorated(self, decorated: bool) -> Self {
        Self {
            builder: self.builder.property("decorated", decorated),
        }
    }

    /// The default height of the window.
    pub fn default_height(self, default_height: i32) -> Self {
        Self {
            builder: self.builder.property("default-height", default_height),
        }
    }

    /// The default widget.
    pub fn default_widget(self, default_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("default-widget", default_widget.clone().upcast()),
        }
    }

    /// The default width of the window.
    pub fn default_width(self, default_width: i32) -> Self {
        Self {
            builder: self.builder.property("default-width", default_width),
        }
    }

    /// Whether the window frame should have a close button.
    pub fn deletable(self, deletable: bool) -> Self {
        Self {
            builder: self.builder.property("deletable", deletable),
        }
    }

    /// If this window should be destroyed when the parent is destroyed.
    pub fn destroy_with_parent(self, destroy_with_parent: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("destroy-with-parent", destroy_with_parent),
        }
    }

    // /// The display that will display this window.
    // pub fn display(self, display: &impl IsA<gdk::Display>) -> Self {
    //     Self {
    //         builder: self.builder.property("display", display.clone().upcast()),
    //     }
    // }

    /// Whether 'focus rectangles' are currently visible in this window.
    ///
    /// This property is maintained by GTK based on user input
    /// and should not be set by applications.
    pub fn focus_visible(self, focus_visible: bool) -> Self {
        Self {
            builder: self.builder.property("focus-visible", focus_visible),
        }
    }

    /// The focus widget.
    pub fn focus_widget(self, focus_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("focus-widget", focus_widget.clone().upcast()),
        }
    }

    /// Whether the window is fullscreen.
    ///
    /// Setting this property is the equivalent of calling
    /// [`GtkSavingWindowExt::fullscreen()`][crate::prelude::GtkSavingWindowExt::fullscreen()] or [`GtkSavingWindowExt::unfullscreen()`][crate::prelude::GtkSavingWindowExt::unfullscreen()];
    /// either operation is asynchronous, which means you will need to
    /// connect to the ::notify signal in order to know whether the
    /// operation was successful.
    pub fn fullscreened(self, fullscreened: bool) -> Self {
        Self {
            builder: self.builder.property("fullscreened", fullscreened),
        }
    }

    /// Whether the window frame should handle F10 for activating
    /// menubars.
    #[cfg(feature = "v4_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_2")))]
    pub fn handle_menubar_accel(self, handle_menubar_accel: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("handle-menubar-accel", handle_menubar_accel),
        }
    }

    /// If this window should be hidden when the users clicks the close button.
    pub fn hide_on_close(self, hide_on_close: bool) -> Self {
        Self {
            builder: self.builder.property("hide-on-close", hide_on_close),
        }
    }

    /// Specifies the name of the themed icon to use as the window icon.
    ///
    /// See [`IconTheme`][crate::IconTheme] for more details.
    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    /// Whether the window is maximized.
    ///
    /// Setting this property is the equivalent of calling
    /// [`GtkSavingWindowExt::maximize()`][crate::prelude::GtkSavingWindowExt::maximize()] or [`GtkSavingWindowExt::unmaximize()`][crate::prelude::GtkSavingWindowExt::unmaximize()];
    /// either operation is asynchronous, which means you will need to
    /// connect to the ::notify signal in order to know whether the
    /// operation was successful.
    pub fn maximized(self, maximized: bool) -> Self {
        Self {
            builder: self.builder.property("maximized", maximized),
        }
    }

    /// Whether mnemonics are currently visible in this window.
    ///
    /// This property is maintained by GTK based on user input,
    /// and should not be set by applications.
    pub fn mnemonics_visible(self, mnemonics_visible: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("mnemonics-visible", mnemonics_visible),
        }
    }

    /// If [`true`], the window is modal.
    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    /// If [`true`], users can resize the window.
    pub fn resizable(self, resizable: bool) -> Self {
        Self {
            builder: self.builder.property("resizable", resizable),
        }
    }

    /// A write-only property for setting window's startup notification identifier.
    pub fn startup_id(self, startup_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("startup-id", startup_id.into()),
        }
    }

    /// The title of the window.
    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    /// The titlebar widget.
    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    pub fn titlebar(self, titlebar: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("titlebar", titlebar.clone().upcast()),
        }
    }

    /// The transient parent of the window.
    pub fn transient_for(self, transient_for: &impl IsA<Window>) -> Self {
        Self {
            builder: self
                .builder
                .property("transient-for", transient_for.clone().upcast()),
        }
    }

    /// Whether the widget or any of its descendents can accept
    /// the input focus.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    /// Whether the widget can receive pointer events.
    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    /// A list of css classes applied to this widget.
    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    /// The name of this widget in the CSS tree.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    // /// The cursor used by @widget.
    // pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
    //     Self {
    //         builder: self.builder.property("cursor", cursor.clone()),
    //     }
    // }

    /// Whether the widget should grab focus when it is clicked with the mouse.
    ///
    /// This property is only relevant for widgets that can take focus.
    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    /// Whether this widget itself will accept the input focus.
    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    /// How to distribute horizontal space if widget gets extra space.
    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    /// Enables or disables the emission of the ::query-tooltip signal on @widget.
    ///
    /// A value of [`true`] indicates that @widget can have a tooltip, in this case
    /// the widget will be queried using [`query-tooltip`][struct@crate::Widget#query-tooltip] to
    /// determine whether it will provide a tooltip or not.
    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    /// Override for height request of the widget.
    ///
    /// If this is -1, the natural request will be used.
    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    /// Whether to expand horizontally.
    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    /// Whether to use the `hexpand` property.
    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    /// The [`LayoutManager`][crate::LayoutManager] instance to use to compute the preferred size
    /// of the widget, and allocate its children.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    /// Margin on bottom side of widget.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExt::set_size_request()`][crate::prelude::WidgetExt::set_size_request()] for example.
    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    /// Margin on end of widget, horizontally.
    ///
    /// This property supports left-to-right and right-to-left text
    /// directions.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExt::set_size_request()`][crate::prelude::WidgetExt::set_size_request()] for example.
    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    /// Margin on start of widget, horizontally.
    ///
    /// This property supports left-to-right and right-to-left text
    /// directions.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExt::set_size_request()`][crate::prelude::WidgetExt::set_size_request()] for example.
    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    /// Margin on top side of widget.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExt::set_size_request()`][crate::prelude::WidgetExt::set_size_request()] for example.
    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    /// The name of the widget.
    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    /// The requested opacity of the widget.
    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    /// How content outside the widget's content area is treated.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    /// Whether the widget will receive the default action when it is focused.
    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    /// Whether the widget responds to input.
    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    /// Sets the text of tooltip to be the given string, which is marked up
    /// with Pango markup.
    ///
    /// Also see [`Tooltip::set_markup()`][crate::Tooltip::set_markup()].
    ///
    /// This is a convenience property which will take care of getting the
    /// tooltip shown if the given string is not [`None`]:
    /// [`has-tooltip`][struct@crate::Widget#has-tooltip] will automatically be set to [`true`]
    /// and there will be taken care of [`query-tooltip`][struct@crate::Widget#query-tooltip] in
    /// the default signal handler.
    ///
    /// Note that if both [`tooltip-text`][struct@crate::Widget#tooltip-text] and
    /// [`tooltip-markup`][struct@crate::Widget#tooltip-markup] are set, the last one wins.
    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    /// Sets the text of tooltip to be the given string.
    ///
    /// Also see [`Tooltip::set_text()`][crate::Tooltip::set_text()].
    ///
    /// This is a convenience property which will take care of getting the
    /// tooltip shown if the given string is not [`None`]:
    /// [`has-tooltip`][struct@crate::Widget#has-tooltip] will automatically be set to [`true`]
    /// and there will be taken care of [`query-tooltip`][struct@crate::Widget#query-tooltip] in
    /// the default signal handler.
    ///
    /// Note that if both [`tooltip-text`][struct@crate::Widget#tooltip-text] and
    /// [`tooltip-markup`][struct@crate::Widget#tooltip-markup] are set, the last one wins.
    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    /// How to distribute vertical space if widget gets extra space.
    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    /// Whether to expand vertically.
    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    /// Whether to use the `vexpand` property.
    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    /// Whether the widget is visible.
    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    /// Override for width request of the widget.
    ///
    /// If this is -1, the natural request will be used.
    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    /// The accessible role of the given [`Accessible`][crate::Accessible] implementation.
    ///
    /// The accessible role cannot be changed once set.
    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SavingWindow`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self, settings: Settings) -> super::SavingWindow {
        let new = self.builder.property("settings", settings);
        new.build()
    }
}