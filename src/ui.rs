use crate::network;
use crate::tray;
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;
use sysinfo::Networks;
use tray_icon::{MouseButton, MouseButtonState, TrayIcon, TrayIconEvent, menu::MenuEvent};

#[derive(Clone)]
struct Dashboard {
    window: gtk4::ApplicationWindow,
    interface_picker: gtk4::DropDown,
    pause_button: gtk4::Button,
    timer_counter: gtk4::Label,
    download_speed: gtk4::Label,
    upload_speed: gtk4::Label,
    download_total: gtk4::Label,
    upload_total: gtk4::Label,
    speed_graph: gtk4::DrawingArea,
}

#[derive(Clone, Default)]
struct ThemeState {
    dark: Rc<Cell<bool>>,
    windows: Rc<RefCell<Vec<gtk4::glib::WeakRef<gtk4::ApplicationWindow>>>>,
    graphs: Rc<RefCell<Vec<gtk4::glib::WeakRef<gtk4::DrawingArea>>>>,
    buttons: Rc<RefCell<Vec<(gtk4::glib::WeakRef<gtk4::Button>, bool)>>>,
}

pub fn ui(tray_icon: TrayIcon) {
    let app = gtk4::Application::builder()
        .application_id("com.example.NetworkMonitor")
        .build();

    app.connect_activate(move |app| {
        let _tray_icon = tray_icon.clone();
        setting();
        let interfaces = Rc::new(network::get_interface());
        let samples = Rc::new(RefCell::new(Vec::new()));
        let chart_times = Rc::new(RefCell::new(Vec::new()));
        refresh_chart_times(&chart_times);
        let paused = Rc::new(Cell::new(false));
        let elapsed = Rc::new(Cell::new(0_u64));
        let theme = ThemeState::default();
        let main = dashboard(
            app,
            &interfaces,
            Rc::clone(&samples),
            Rc::clone(&paused),
            theme.clone(),
            Rc::clone(&chart_times),
        );
        let mini = mini_dashboard(
            app,
            &interfaces,
            Rc::clone(&samples),
            Rc::clone(&paused),
            theme.clone(),
            Rc::clone(&chart_times),
        );
        bind_interface_selection(&main.interface_picker, &mini.interface_picker);
        bind_pause_buttons(&main.pause_button, &mini.pause_button, Rc::clone(&paused));
        sync_theme(&theme);
        bind_close_to_hide(&main.window);
        bind_close_to_hide(&mini.window);
        main.window.present();

        let networks = Rc::new(RefCell::new(Networks::new_with_refreshed_list()));
        timer(
            interfaces,
            networks,
            paused,
            elapsed,
            [main.clone(), mini.clone()],
            samples,
            chart_times,
        );

        let main_window = main.window.clone();
        let mini_window = mini.window.clone();
        let app = app.clone();
        gtk4::glib::timeout_add_local(Duration::from_millis(100), move || {
            for event in MenuEvent::receiver().try_iter() {
                if event.id() == tray::OPEN_APP_ID {
                    mini_window.hide();
                    main_window.present();
                } else if event.id() == tray::EXIT_ID {
                    app.quit();
                }
            }

            for event in TrayIconEvent::receiver().try_iter() {
                if event.id() != _tray_icon.id() {
                    continue;
                }

                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    if mini_window.is_visible() {
                        mini_window.hide();
                    } else {
                        mini_window.present();
                    }
                }
            }

            gtk4::glib::ControlFlow::Continue
        });
    });

    app.run();
}

fn dashboard(
    app: &gtk4::Application,
    interfaces: &[String],
    samples: Rc<RefCell<Vec<(f64, f64)>>>,
    paused: Rc<Cell<bool>>,
    theme: ThemeState,
    chart_times: Rc<RefCell<Vec<String>>>,
) -> Dashboard {
    let interface_picker = drop_down(interfaces);
    let download_speed = text_ui("0 MB/s");
    let upload_speed = text_ui("0 MB/s");
    let download_total = secondary_text("0.00 B");
    let upload_total = secondary_text("0.00 B");
    let timer_counter = text_ui("00:00");
    timer_counter.add_css_class("muted");
    let speed_graph = graph(Rc::clone(&samples), 220, theme.clone(), chart_times);
    let pause_button = gtk4::Button::with_label(if paused.get() { "Resume" } else { "Pause" });
    pause_button.add_css_class("primary-button");
    pause_button.set_size_request(120, 38);

    let window = gtk4::ApplicationWindow::builder()
        .application(app)
        .title("Network Manager")
        .default_width(900)
        .default_height(620)
        .resizable(false)
        .build();
    window.add_css_class("network-window");
    theme.windows.borrow_mut().push(window.downgrade());

    let workspace = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    workspace.set_margin_top(20);
    workspace.set_margin_bottom(18);
    workspace.set_margin_start(20);
    workspace.set_margin_end(20);
    workspace.set_hexpand(true);

    let topbar = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    interface_picker.set_size_request(205, 42);
    interface_picker.set_hexpand(true);
    topbar.append(&interface_picker);
    topbar.append(&theme_switch(theme.clone()));
    workspace.append(&topbar);

    let graph_card = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    graph_card.add_css_class("surface-card");
    graph_card.set_hexpand(true);
    let chart_header = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    chart_header.append(&section_heading("Network Activity"));
    let legend = graph_legend();
    chart_header.append(&legend);
    graph_card.append(&chart_header);
    graph_card.append(&speed_graph);
    workspace.append(&graph_card);

    let metrics = gtk4::Grid::new();
    metrics.set_column_spacing(12);
    metrics.set_column_homogeneous(true);
    metrics.attach(
        &metric_card("Download Speed", "arrow-down-symbolic", &download_speed),
        0,
        0,
        1,
        1,
    );
    metrics.attach(
        &metric_card("Upload Speed", "arrow-up-symbolic", &upload_speed),
        1,
        0,
        1,
        1,
    );
    metrics.attach(
        &metric_card(
            "Total Downloaded",
            "folder-download-symbolic",
            &download_total,
        ),
        2,
        0,
        1,
        1,
    );
    metrics.attach(
        &metric_card("Total Uploaded", "folder-upload-symbolic", &upload_total),
        3,
        0,
        1,
        1,
    );
    workspace.append(&metrics);

    let footer = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    footer.add_css_class("footer");
    footer.append(&pause_button);
    let footer_status = gtk4::Label::new(Some("Monitoring"));
    footer_status.add_css_class("muted");
    footer_status.set_hexpand(true);
    footer_status.set_xalign(1.0);
    footer.append(&footer_status);
    footer.append(&timer_counter);
    footer.append(&version_label());
    workspace.append(&footer);
    window.set_child(Some(&workspace));

    theme.graphs.borrow_mut().push(speed_graph.downgrade());

    Dashboard {
        window,
        interface_picker,
        pause_button,
        timer_counter,
        download_speed,
        upload_speed,
        download_total,
        upload_total,
        speed_graph,
    }
}

fn mini_dashboard(
    app: &gtk4::Application,
    interfaces: &[String],
    samples: Rc<RefCell<Vec<(f64, f64)>>>,
    paused: Rc<Cell<bool>>,
    theme: ThemeState,
    chart_times: Rc<RefCell<Vec<String>>>,
) -> Dashboard {
    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 14);
    content.set_margin_top(18);
    content.set_margin_bottom(16);
    content.set_margin_start(18);
    content.set_margin_end(18);
    content.add_css_class("mini-shell");

    let header = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    header.append(&brand_header());
    let header_spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    header_spacer.set_hexpand(true);
    header.append(&header_spacer);
    header.append(&theme_switch(theme.clone()));
    let close = gtk4::Button::from_icon_name("window-close-symbolic");
    close.add_css_class("flat-button");
    close.set_tooltip_text(Some("Hide mini dashboard"));
    header.append(&close);

    let download_speed = text_ui("0 MB/s");
    let upload_speed = text_ui("0 MB/s");
    let download_total = secondary_text("Total: 0 MB");
    let upload_total = secondary_text("Total: 0 MB");
    let timer_counter = text_ui("00:00");
    timer_counter.add_css_class("muted");
    let speed_graph = graph(Rc::clone(&samples), 150, theme.clone(), chart_times);
    let graph_panel = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    graph_panel.add_css_class("surface-card");
    let graph_header = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    graph_header.append(&section_heading("Network Activity"));
    let legend = graph_legend();
    graph_header.append(&legend);
    graph_panel.append(&graph_header);
    graph_panel.append(&speed_graph);

    let metrics = gtk4::Grid::new();
    metrics.set_column_spacing(10);
    metrics.set_row_spacing(8);
    metrics.set_column_homogeneous(true);
    metrics.attach(
        &metric_card("Download Speed", "arrow-down-symbolic", &download_speed),
        0,
        0,
        1,
        1,
    );
    metrics.attach(
        &metric_card("Upload Speed", "arrow-up-symbolic", &upload_speed),
        1,
        0,
        1,
        1,
    );
    metrics.attach(
        &metric_card(
            "Total Downloaded",
            "folder-download-symbolic",
            &download_total,
        ),
        0,
        1,
        1,
        1,
    );
    metrics.attach(
        &metric_card("Total Uploaded", "folder-upload-symbolic", &upload_total),
        1,
        1,
        1,
        1,
    );

    let interface_picker = drop_down(interfaces);
    interface_picker.set_size_request(165, 38);
    interface_picker.set_hexpand(true);
    let pause_button = gtk4::Button::with_label(if paused.get() { "Resume" } else { "Pause" });
    pause_button.add_css_class("primary-button");
    let footer = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    footer.append(&interface_picker);
    footer.append(&pause_button);
    footer.append(&timer_counter);
    footer.append(&version_label());

    let body = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    body.set_margin_top(6);
    body.append(&graph_panel);
    body.append(&metrics);
    body.append(&footer);
    content.append(&header);
    content.append(&body);

    let window = gtk4::ApplicationWindow::builder()
        .application(app)
        .title("Network Monitor")
        .default_width(500)
        .default_height(520)
        .decorated(true)
        .modal(false)
        .resizable(false)
        .child(&content)
        .build();
    window.add_css_class("mini-window");
    window.add_css_class("network-window");
    theme.windows.borrow_mut().push(window.downgrade());
    theme.graphs.borrow_mut().push(speed_graph.downgrade());

    let window_for_close = window.downgrade();
    close.connect_clicked(move |_| {
        if let Some(window) = window_for_close.upgrade() {
            window.hide();
        }
    });

    Dashboard {
        window,
        interface_picker,
        pause_button,
        timer_counter,
        download_speed,
        upload_speed,
        download_total,
        upload_total,
        speed_graph,
    }
}

fn section_heading(text: &str) -> gtk4::Label {
    let heading = gtk4::Label::new(Some(text));
    heading.set_xalign(0.0);
    heading.add_css_class("section-heading");
    heading
}

fn metric_card(name: &str, icon_name: &str, value: &gtk4::Label) -> gtk4::Box {
    let card = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    card.add_css_class("metric-card");

    let label_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 7);
    let icon = gtk4::Image::from_icon_name(icon_name);
    icon.set_pixel_size(20);
    icon.add_css_class("metric-icon");
    if name.starts_with("Upload") || name.starts_with("Total Uploaded") {
        icon.add_css_class("upload");
    } else {
        icon.add_css_class("download");
    }
    let label = gtk4::Label::new(Some(name));
    label.set_xalign(0.0);
    label.set_wrap(true);
    label.set_hexpand(true);
    label.add_css_class("metric-name");
    label_row.append(&icon);
    label_row.append(&label);

    value.add_css_class("metric-value");
    value.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    card.append(&label_row);
    card.append(value);
    card
}

fn brand_header() -> gtk4::Box {
    let brand = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    let icon = gtk4::Image::from_icon_name("network-wireless-symbolic");
    icon.set_pixel_size(34);
    icon.add_css_class("brand-icon");
    let brand_text = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let name = gtk4::Label::new(Some("Network Manager"));
    name.set_xalign(0.0);
    name.add_css_class("brand-name");
    let version = version_label();
    version.set_xalign(0.0);
    version.add_css_class("brand-header-version");
    brand_text.append(&name);
    brand_text.append(&version);
    brand.append(&icon);
    brand.append(&brand_text);
    brand
}

fn graph_legend() -> gtk4::Box {
    let legend = gtk4::Box::new(gtk4::Orientation::Horizontal, 14);
    legend.set_hexpand(true);
    legend.set_halign(gtk4::Align::End);
    for (label, color_class) in [("Download", "download"), ("Upload", "upload")] {
        let item = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        let dot = gtk4::Image::from_icon_name("media-record-symbolic");
        dot.set_pixel_size(9);
        dot.add_css_class(color_class);
        let label = gtk4::Label::new(Some(label));
        label.add_css_class("muted");
        item.append(&dot);
        item.append(&label);
        legend.append(&item);
    }
    legend
}

fn theme_switch(theme: ThemeState) -> gtk4::Box {
    let switch = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    switch.add_css_class("theme-switch");
    let light = gtk4::Button::from_icon_name("weather-clear-symbolic");
    let dark = gtk4::Button::from_icon_name("weather-clear-night-symbolic");
    light.add_css_class("theme-choice");
    dark.add_css_class("theme-choice");
    light.set_tooltip_text(Some("Light theme"));
    dark.set_tooltip_text(Some("Dark theme"));
    switch.append(&light);
    switch.append(&dark);
    theme.buttons.borrow_mut().push((light.downgrade(), false));
    theme.buttons.borrow_mut().push((dark.downgrade(), true));

    for (button, is_dark) in [(light, false), (dark, true)] {
        let theme = theme.clone();
        button.connect_clicked(move |_| {
            theme.dark.set(is_dark);
            sync_theme(&theme);
        });
    }
    switch
}

fn bind_pause_buttons(main: &gtk4::Button, mini: &gtk4::Button, paused: Rc<Cell<bool>>) {
    let main_weak = main.downgrade();
    let mini_weak = mini.downgrade();
    for button in [main, mini] {
        let paused = Rc::clone(&paused);
        let main = main_weak.clone();
        let mini = mini_weak.clone();
        button.connect_clicked(move |_| {
            paused.set(!paused.get());
            let label = if paused.get() { "Resume" } else { "Pause" };
            if let Some(button) = main.upgrade() {
                button.set_label(label);
            }
            if let Some(button) = mini.upgrade() {
                button.set_label(label);
            }
        });
    }
}

fn sync_theme(theme: &ThemeState) {
    for window in theme
        .windows
        .borrow()
        .iter()
        .filter_map(|window| window.upgrade())
    {
        if theme.dark.get() {
            window.add_css_class("dark-theme");
        } else {
            window.remove_css_class("dark-theme");
        }
    }
    for graph in theme
        .graphs
        .borrow()
        .iter()
        .filter_map(|graph| graph.upgrade())
    {
        graph.queue_draw();
    }
    for (button, is_dark) in theme.buttons.borrow().iter() {
        if let Some(button) = button.upgrade() {
            if theme.dark.get() == *is_dark {
                button.add_css_class("active");
            } else {
                button.remove_css_class("active");
            }
        }
    }
}

fn bind_interface_selection(main: &gtk4::DropDown, mini: &gtk4::DropDown) {
    let mini_weak = mini.downgrade();
    main.connect_selected_notify(move |picker| {
        if let Some(mini) = mini_weak.upgrade() {
            if mini.selected() != picker.selected() {
                mini.set_selected(picker.selected());
            }
        }
    });

    let main_weak = main.downgrade();
    mini.connect_selected_notify(move |picker| {
        if let Some(main) = main_weak.upgrade() {
            if main.selected() != picker.selected() {
                main.set_selected(picker.selected());
            }
        }
    });
}

fn bind_close_to_hide(window: &gtk4::ApplicationWindow) {
    window.connect_close_request(|window| {
        window.hide();
        gtk4::glib::Propagation::Stop
    });
}

fn setting() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(
        "window.network-window { background-color: #f2f5fa; color: #182b48; }
         .mini-shell { background-color: #f2f5fa; color: #182b48; }
         .brand-name { color: #172a47; font-size: 15px; font-weight: 700; }
         .brand-icon { color: #ffffff; background-color: #0878f9; border-radius: 9px; padding: 7px; }
         .muted, .secondary { color: #71819b; }
         .surface-card, .metric-card { background-color: #ffffff; border: 1px solid #dce5f1; border-radius: 12px; padding: 14px; }
         .section-heading { color: #172a47; font-size: 17px; font-weight: 700; }
         .metric-icon.download { color: #0086b3; background-color: alpha(#0096c7, 0.14); border-radius: 999px; padding: 7px; }
         .metric-icon.upload { color: #78ac00; background-color: alpha(#90a955, 0.18); border-radius: 999px; padding: 7px; }
         .metric-name { color: #40536f; font-size: 13px; }
         .metric-value { color: #172a47; font-size: 24px; font-weight: 700; }
         .footer { border-top: 1px solid #dce5f1; padding-top: 10px; }
         button { color: #263b59; background-color: #e8eef7; border: 1px solid #d5dfec; border-radius: 9px; padding: 7px 12px; }
         button:hover { background-color: #dce7f5; }
         button.primary-button { color: #ffffff; background-color: #0878f9; border-color: #0878f9; font-weight: 600; }
         button.primary-button:hover { background-color: #0068e5; }
         button.secondary-button { background-color: #e8eef7; border-color: #d5dfec; }
         button.flat-button { background-color: transparent; border-color: transparent; }
         .interface-picker { color: #263b59; background-color: #ffffff; border: 1px solid #d5dfec; border-radius: 9px; }
         .interface-picker button, .interface-picker button label { color: #263b59; background-color: #ffffff; border: 0; }
         .theme-switch { background-color: #e6edf7; border-radius: 999px; padding: 3px; }
         .theme-switch button { min-width: 34px; min-height: 30px; padding: 3px; border: 0; border-radius: 999px; background-color: transparent; }
         .theme-switch button:hover { background-color: #ffffff; }
         .theme-switch button.active { color: #0878f9; background-color: #ffffff; }
         .download { color: #0086b3; }
         .upload { color: #78ac00; }
         .plot { background-color: #ffffff; }
         .mini-window { border-radius: 14px; }
         .mini-shell { padding: 16px; }
         .mini-shell .brand-icon { padding: 5px; }
         .mini-shell .brand-name { font-size: 13px; }
         .mini-shell .brand-header-version { font-size: 10px; }
         .mini-shell .surface-card { padding: 10px; }
         .mini-shell .section-heading { font-size: 14px; }
         .mini-shell .metric-card { padding: 10px; }
         .mini-shell .metric-value { font-size: 18px; }
         .mini-shell .metric-name { font-size: 11px; }
         .mini-shell .metric-icon { padding: 5px; }
         window.network-window.dark-theme { background-color: #101b2c; color: #e9f1ff; }
         .dark-theme.mini-shell, .dark-theme .mini-shell { background-color: #101b2c; color: #e9f1ff; }
         .dark-theme .brand-name, .dark-theme .section-heading, .dark-theme .metric-value { color: #edf4ff; }
         .dark-theme .surface-card, .dark-theme .metric-card { background-color: #17263b; border-color: #293a52; }
         .dark-theme .metric-name, .dark-theme .muted, .dark-theme .secondary { color: #9aacc4; }
         .dark-theme .metric-icon.download { color: #0096c7; background-color: alpha(#0096c7, 0.24); }
         .dark-theme .metric-icon.upload { color: #90a955; background-color: alpha(#90a955, 0.24); }
         .dark-theme button { color: #e5eefb; background-color: #22334a; border-color: #32455f; }
         .dark-theme button:hover { background-color: #2b405b; }
         .dark-theme button.primary-button { color: #ffffff; background-color: #0878f9; border-color: #0878f9; }
         .dark-theme button.secondary-button { background-color: #22334a; border-color: #32455f; }
         .dark-theme button.flat-button { background-color: transparent; border-color: transparent; }
         .dark-theme .interface-picker, .dark-theme .interface-picker button, .dark-theme .interface-picker button label { color: #e5eefb; background-color: #17263b; border-color: #32455f; }
         .dark-theme .theme-switch { background-color: #26364d; }
         .dark-theme .theme-switch button:hover { background-color: #405570; }
         .dark-theme .theme-switch button.active { color: #69adff; background-color: #405570; }
         .dark-theme .plot { background-color: #17263b; }",
    );

    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn version_label() -> gtk4::Label {
    let version = gtk4::Label::new(Some(&format!("v{}", env!("CARGO_PKG_VERSION"))));
    version.add_css_class("muted");
    version
}

fn graph(
    samples: Rc<RefCell<Vec<(f64, f64)>>>,
    height: i32,
    theme: ThemeState,
    chart_times: Rc<RefCell<Vec<String>>>,
) -> gtk4::DrawingArea {
    let area = gtk4::DrawingArea::new();
    area.set_content_height(height);
    area.set_hexpand(true);
    area.add_css_class("plot");

    area.set_draw_func(move |_, context, width, height| {
        let samples = samples.borrow();
        let top_padding = 16.0;
        let bottom_padding = 24.0;
        let side_padding = 18.0;
        let y_axis_label_width = 54.0;
        let plot_left = side_padding + y_axis_label_width;
        let chart_width = (width as f64 - plot_left - side_padding).max(1.0);
        let chart_height = (height as f64 - top_padding - bottom_padding).max(1.0);
        let max_value = samples
            .iter()
            .flat_map(|(download, upload)| [*download, *upload])
            .fold(0.0_f64, f64::max)
            .max(1.0);
        let maximum = chart_scale(max_value);
        let axis_color = if theme.dark.get() {
            (0.52, 0.61, 0.73, 0.45)
        } else {
            (0.42, 0.51, 0.63, 0.35)
        };
        let graph_colors = (
            (
                0x00 as f64 / 255.0,
                0x96 as f64 / 255.0,
                0xC7 as f64 / 255.0,
            ),
            (
                0x90 as f64 / 255.0,
                0xA9 as f64 / 255.0,
                0x55 as f64 / 255.0,
            ),
        );

        context.set_line_width(1.0);
        context.set_source_rgba(axis_color.0, axis_color.1, axis_color.2, axis_color.3);
        context.set_font_size(10.0);
        for row in 0..=4 {
            let y = top_padding + chart_height * row as f64 / 4.0;
            context.move_to(plot_left, y);
            context.line_to(plot_left + chart_width, y);
            let _ = context.stroke();

            let label = format_speed(maximum * (1.0 - row as f64 / 4.0));
            context.move_to(4.0, y + 4.0);
            let _ = context.show_text(&label);
        }

        let axis_start = top_padding + chart_height;
        context.move_to(plot_left, axis_start);
        context.line_to(plot_left + chart_width, axis_start);
        let _ = context.stroke();
        context.set_source_rgba(axis_color.0, axis_color.1, axis_color.2, 1.0);
        context.set_font_size(9.0);
        for (index, label) in chart_times.borrow().iter().enumerate() {
            let x = plot_left + chart_width * index as f64 / 6.0;
            let text_width = context
                .text_extents(label)
                .map_or(0.0, |extents| extents.width());
            context.move_to(x - text_width / 2.0, height as f64 - 5.0);
            let _ = context.show_text(label);
        }

        draw_speed_line(
            context,
            &samples,
            0,
            maximum,
            chart_width,
            chart_height,
            top_padding,
            plot_left,
            graph_colors.0,
        );
        draw_speed_line(
            context,
            &samples,
            1,
            maximum,
            chart_width,
            chart_height,
            top_padding,
            plot_left,
            graph_colors.1,
        );
    });

    area
}

fn chart_scale(value: f64) -> f64 {
    if value <= 1.0 {
        return 1.0;
    }

    let mut threshold = 1.0;
    let mut next = 2.0;
    while value > threshold {
        threshold = next;
        next = if threshold <= 2.0 {
            5.0
        } else if threshold <= 5.0 {
            10.0
        } else {
            threshold + 5.0
        };
        if value <= threshold {
            return threshold;
        }
    }

    threshold
}

fn format_speed(value: f64) -> String {
    if value < 1.0 {
        format!("{:.0} KB/s", value * 1024.0)
    } else if value < 10.0 {
        format!("{value:.1} MB/s")
    } else {
        format!("{value:.0} MB/s")
    }
}

fn text_ui(text: &str) -> gtk4::Label {
    let label = gtk4::Label::new(Some(text));
    label.set_xalign(0.0);
    label.add_css_class("metric");
    label
}

fn secondary_text(text: &str) -> gtk4::Label {
    let label = gtk4::Label::new(Some(text));
    label.set_xalign(0.0);
    label.add_css_class("secondary");
    label
}

fn drop_down(interfaces: &[String]) -> gtk4::DropDown {
    let names: Vec<&str> = interfaces.iter().map(String::as_str).collect();
    let dropdown = gtk4::DropDown::from_strings(&names);
    dropdown.add_css_class("interface-picker");
    dropdown.set_size_request(0, 38);
    if !interfaces.is_empty() {
        dropdown.set_selected(0);
    }
    dropdown
}

fn draw_speed_line(
    context: &gtk4::cairo::Context,
    samples: &[(f64, f64)],
    index: usize,
    maximum: f64,
    chart_width: f64,
    chart_height: f64,
    top_padding: f64,
    left_padding: f64,
    color: (f64, f64, f64),
) {
    if samples.is_empty() {
        return;
    }

    let points: Vec<(f64, f64)> = samples
        .iter()
        .enumerate()
        .map(|(point, sample)| {
            let speed = if index == 0 { sample.0 } else { sample.1 };
            let x = left_padding
                + chart_width * point as f64 / samples.len().saturating_sub(1).max(1) as f64;
            let y = top_padding + chart_height * (1.0 - (speed / maximum).clamp(0.0, 1.0));
            (x, y)
        })
        .collect();

    if points.is_empty() {
        return;
    }

    context.set_source_rgb(color.0, color.1, color.2);
    context.set_line_width(2.2);
    context.new_path();
    context.move_to(points[0].0, points[0].1);

    if points.len() > 1 {
        for index in 0..points.len() - 1 {
            let previous = points[index.saturating_sub(1)];
            let current = points[index];
            let next = points[index + 1];
            let after_next = points.get(index + 2).copied().unwrap_or(next);
            let control_1 = (
                current.0 + (next.0 - previous.0) / 6.0,
                current.1 + (next.1 - previous.1) / 6.0,
            );
            let control_2 = (
                next.0 - (after_next.0 - current.0) / 6.0,
                next.1 - (after_next.1 - current.1) / 6.0,
            );
            context.curve_to(
                control_1.0,
                control_1.1,
                control_2.0,
                control_2.1,
                next.0,
                next.1,
            );
        }
    }

    let baseline = top_padding + chart_height;
    context.line_to(
        points
            .last()
            .map(|(x, _)| *x)
            .unwrap_or(left_padding + chart_width),
        baseline,
    );
    context.line_to(points[0].0, baseline);
    context.close_path();

    let gradient =
        gtk4::cairo::LinearGradient::new(0.0, top_padding, 0.0, top_padding + chart_height);
    gradient.add_color_stop_rgba(0.0, color.0, color.1, color.2, 0.20);
    gradient.add_color_stop_rgba(1.0, color.0, color.1, color.2, 0.02);
    let _ = context.set_source(&gradient);
    let _ = context.fill_preserve();
    context.set_source_rgb(color.0, color.1, color.2);
    let _ = context.stroke();
}

fn format_timer(seconds: u64) -> String {
    let minutes = seconds / 60;
    let secs = seconds % 60;
    format!("{minutes:02}:{secs:02}")
}

fn refresh_chart_times(chart_times: &RefCell<Vec<String>>) {
    match system_clock_labels() {
        Ok(labels) => *chart_times.borrow_mut() = labels,
        Err(error) => eprintln!("Unable to read local system time for chart: {error}"),
    }
}

fn system_clock_labels() -> Result<Vec<String>, gtk4::glib::BoolError> {
    let now = gtk4::glib::DateTime::now_local()?;
    (0..=6)
        .map(|tick| {
            let time = now.add_seconds(-60.0 + tick as f64 * 10.0)?;
            Ok(time.format("%H:%M:%S")?.to_string())
        })
        .collect()
}

fn timer(
    interfaces: Rc<Vec<String>>,
    networks: Rc<RefCell<Networks>>,
    paused: Rc<Cell<bool>>,
    elapsed: Rc<Cell<u64>>,
    dashboards: [Dashboard; 2],
    samples: Rc<RefCell<Vec<(f64, f64)>>>,
    chart_times: Rc<RefCell<Vec<String>>>,
) {
    gtk4::glib::timeout_add_local(Duration::from_secs(1), move || {
        refresh_chart_times(&chart_times);
        for dashboard in &dashboards {
            dashboard.speed_graph.queue_draw();
        }

        if !paused.get() {
            let next = elapsed.get() + 1;
            elapsed.set(next);
            for dashboard in &dashboards {
                dashboard.timer_counter.set_text(&format_timer(next));
            }
        }

        if paused.get() {
            return gtk4::glib::ControlFlow::Continue;
        }

        let selected = dashboards[0].interface_picker.selected() as usize;
        let Some(interface_name) = interfaces.get(selected) else {
            return gtk4::glib::ControlFlow::Continue;
        };

        let readings = {
            let mut networks = networks.borrow_mut();
            network::network(&mut networks)
        };

        if let Some(values) = readings.get(interface_name) {
            let (Some(total_downloaded), Some(total_uploaded)) = (
                values.get("AD").and_then(|value| value.parse::<u64>().ok()),
                values.get("AU").and_then(|value| value.parse::<u64>().ok()),
            ) else {
                return gtk4::glib::ControlFlow::Continue;
            };
            let (Some(download), Some(upload)) = (values.get("SD"), values.get("SU")) else {
                return gtk4::glib::ControlFlow::Continue;
            };
            let (Some(download_rate), Some(upload_rate)) =
                (parse_speed_mb(download), parse_speed_mb(upload))
            else {
                return gtk4::glib::ControlFlow::Continue;
            };

            let download_speed = format!("{download}/s");
            let upload_speed = format!("{upload}/s");
            let download_total = format_bytes(total_downloaded);
            let upload_total = format_bytes(total_uploaded);
            for dashboard in &dashboards {
                dashboard.download_speed.set_text(&download_speed);
                dashboard.upload_speed.set_text(&upload_speed);
                dashboard.download_total.set_text(&download_total);
                dashboard.upload_total.set_text(&upload_total);
            }

            let mut samples = samples.borrow_mut();
            samples.push((download_rate, upload_rate));
            if samples.len() > 60 {
                samples.remove(0);
            }
            drop(samples);
            for dashboard in &dashboards {
                dashboard.speed_graph.queue_draw();
            }
        }

        gtk4::glib::ControlFlow::Continue
    });
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1_000.0 && unit < UNITS.len() - 1 {
        value /= 1_000.0;
        unit += 1;
    }
    format!("{value:.2} {}", UNITS[unit])
}

fn parse_speed_mb(speed: &str) -> Option<f64> {
    let mut parts = speed.split_whitespace();
    let value = parts.next()?.parse::<f64>().ok()?;
    let unit = parts.next()?;
    if parts.next().is_some() || !value.is_finite() || value < 0.0 {
        return None;
    }

    let bytes_per_unit = match unit {
        "B" => 1.0,
        "KB" => 1024.0,
        "MB" => 1024.0_f64.powi(2),
        "GB" => 1024.0_f64.powi(3),
        "TB" => 1024.0_f64.powi(4),
        _ => return None,
    };

    Some(value * bytes_per_unit / 1024.0_f64.powi(2))
}

#[cfg(test)]
mod tests {
    use super::{parse_speed_mb, system_clock_labels};

    #[test]
    fn chart_times_are_seven_local_clock_ticks() {
        let labels = system_clock_labels().expect("local system clock should be available");

        assert_eq!(labels.len(), 7);
        assert!(labels.iter().all(|label| label.len() == 8
            && label.chars().nth(2) == Some(':')
            && label.chars().nth(5) == Some(':')));
    }

    #[test]
    fn speed_units_are_converted_for_the_graph() {
        assert_eq!(parse_speed_mb("512 B"), Some(512.0 / 1024.0_f64.powi(2)));
        assert_eq!(parse_speed_mb("1.00 KB"), Some(1.0 / 1024.0));
        assert_eq!(
            parse_speed_mb("1.00 MB"),
            Some(1.0)
        );
        assert_eq!(
            parse_speed_mb("1.00 GB"),
            Some(1024.0)
        );
        assert_eq!(
            parse_speed_mb("1.00 TB"),
            Some(1024.0_f64.powi(2))
        );
        assert_eq!(parse_speed_mb("invalid"), None);
    }
}
