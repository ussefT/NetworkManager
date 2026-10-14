use crate::network;
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;
use sysinfo::Networks;

pub fn ui() {
    let app = gtk4::Application::builder()
        .application_id("com.example.NetworkMonitor")
        .build();

    app.connect_activate(|app| {
        let version = setting();
        let interfaces = Rc::new(network::get_interface());
        let interface_picker = drop_down(&interfaces);
        let download_speed = text_ui("0 MB/s");
        let upload_speed = text_ui("0 MB/s");
        let download_total = secondary_text("Total: 0 MB");
        let upload_total = secondary_text("Total: 0 MB");
        let timer_counter = text_ui("00:00");
        let samples = Rc::new(RefCell::new(Vec::new()));
        let speed_graph = graph(Rc::clone(&samples));
        let paused = Rc::new(Cell::new(false));
        let elapsed = Rc::new(Cell::new(0_u64));
        let pause_button = btn_ui(Rc::clone(&paused));

        let content = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
        content.set_margin_top(24);
        content.set_margin_bottom(24);
        content.set_margin_start(24);
        content.set_margin_end(24);
        content.set_halign(gtk4::Align::Center);
        content.add_css_class("dashboard");

        let window = gtk4::ApplicationWindow::builder()
            .application(app)
            .title("Network monitor")
            .default_width(820)
            .default_height(560)
            .resizable(false)
            .child(&content)
            .build();
        window.add_css_class("theme-window");
        window.add_css_class("light-theme");
        content.add_css_class("light-theme");

        let title = gtk4::Label::new(Some("Network monitor"));
        title.set_xalign(0.0);
        title.add_css_class("heading");

        let header = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        version.set_hexpand(true);
        version.set_xalign(1.0);
        timer_counter.set_xalign(1.0);
        timer_counter.set_hexpand(false);
        header.append(&title);
        header.append(&version);
        header.append(&timer_counter);

        let controls = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        interface_picker.set_hexpand(true);

        controls.append(&interface_picker);
        controls.append(&pause_button);

        let traffic = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        traffic.add_css_class("panel");
        traffic.append(&traffic_column(
            "Download",
            &download_speed,
            &download_total,
        ));
        traffic.append(&gtk4::Separator::new(gtk4::Orientation::Vertical));
        traffic.append(&traffic_column("Upload", &upload_speed, &upload_total));

        content.append(&header);
        content.append(&controls);
        content.append(&traffic);
        content.append(&speed_graph);

        window.present();

        let networks = Rc::new(RefCell::new(Networks::new_with_refreshed_list()));
        timer(
            interfaces,
            interface_picker,
            networks,
            paused,
            elapsed,
            timer_counter,
            download_speed,
            upload_speed,
            download_total,
            upload_total,
            samples,
            speed_graph,
        );
    });

    app.run();
}

fn setting() -> gtk4::Label {
    if let Some(settings) = gtk4::Settings::default() {
        settings.set_gtk_application_prefer_dark_theme(false);
    }

    let provider = gtk4::CssProvider::new();
    provider.load_from_data(
        "window, window.background { background-color: #f5f6f8; color: #202733; }
         .light-theme, .light-theme * { color: #202733; }
         .dashboard { min-width: 600px; background-color: #f5f6f8; }
         .panel { background-color: #ffffff; border: 1px solid #dce1e8; border-radius: 12px; padding: 16px; }
         .heading { color: #202733; font-size: 26px; font-weight: 700; }
         .muted { color: #566273; }
         .metric { color: #202733; font-size: 16px; font-weight: 700; }
         .secondary { color: #657184; font-size: 14px; }
         .traffic-column { padding: 18px; }
         button { background-color: #e9edf2; color: #202733; border: 1px solid #cbd3dd; border-radius: 10px; padding: 8px 14px; }
         button:hover { background-color: #dfe5ec; color: #202733; }
         button:active, button:checked { background-color: #d5dce5; color: #202733; }
         button:disabled { background-color: #eef0f3; color: #778292; }
         button:backdrop, button:backdrop label { color: #202733; }
         .interface-picker { background-color: #ffffff; color: #202733; border: 1px solid #cbd3dd; border-radius: 10px; }
         .interface-picker button, .interface-picker button label { background-color: #ffffff; color: #202733; border: 0; }
         .interface-picker popover, .interface-picker popover contents { background-color: #ffffff; color: #202733; }
         .interface-picker list, .interface-picker listview, .interface-picker row { background-color: #ffffff; color: #202733; }
         .interface-picker row:hover, .interface-picker row:selected { background-color: #e9edf2; color: #202733; }
         separator { background-color: #dce1e8; }",
    );

    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    let version = gtk4::Label::new(Some(&format!("v{}", env!("CARGO_PKG_VERSION"))));
    version.add_css_class("muted");
    version
}

fn graph(samples: Rc<RefCell<Vec<(f64, f64)>>>) -> gtk4::DrawingArea {
    let area = gtk4::DrawingArea::new();
    area.set_content_height(220);
    area.set_hexpand(true);
    area.add_css_class("panel");

    area.set_draw_func(move |_, context, width, height| {
        let samples = samples.borrow();
        let top_padding = 38.0;
        let bottom_padding = 18.0;
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
        let axis_color = (0.25, 0.29, 0.35, 1.0);
        let graph_colors = (
            (
                0x6A as f64 / 255.0,
                0x04 as f64 / 255.0,
                0x0F as f64 / 255.0,
            ),
            (
                0xFF as f64 / 255.0,
                0xBA as f64 / 255.0,
                0x08 as f64 / 255.0,
            ),
        );

        context.set_font_size(11.0);
        for (x, color, label) in [
            (plot_left + 4.0, graph_colors.0, "Download"),
            (plot_left + 94.0, graph_colors.1, "Upload"),
        ] {
            context.set_source_rgb(color.0, color.1, color.2);
            context.arc(x + 4.0, 18.0, 4.0, 0.0, std::f64::consts::TAU);
            let _ = context.fill();
            context.move_to(x + 13.0, 22.0);
            let _ = context.show_text(label);
        }

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
        format!("{:.0} KB/s", value * 1000.0)
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

fn btn_ui(paused: Rc<Cell<bool>>) -> gtk4::Button {
    let button = gtk4::Button::with_label("Pause");
    button.set_size_request(120, 38);
    button.connect_clicked(move |button| {
        let is_paused = !paused.get();
        paused.set(is_paused);
        button.set_label(if is_paused { "Resume" } else { "Pause" });
    });
    button
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

fn traffic_column(title: &str, speed: &gtk4::Label, total: &gtk4::Label) -> gtk4::Box {
    let column = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    column.set_hexpand(true);
    column.add_css_class("traffic-column");

    let heading = gtk4::Label::new(Some(title));
    heading.set_xalign(0.0);
    heading.add_css_class("muted");
    column.append(&heading);
    column.append(speed);
    column.append(total);
    column
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
        for i in 1..points.len() - 1 {
            let prev = points[i - 1];
            let current = points[i];
            let next = points[i + 1];
            let cp1x = current.0 + (next.0 - prev.0) / 6.0;
            let cp1y = current.1 + (next.1 - prev.1) / 6.0;
            let next_after = points.get(i + 2).unwrap_or(&next);
            let cp2x = next.0 - (next_after.0 - current.0) / 6.0;
            let cp2y = next.1 - (next_after.1 - current.1) / 6.0;
            context.curve_to(cp1x, cp1y, cp2x, cp2y, next.0, next.1);
        }

        let last = *points.last().unwrap();
        context.line_to(last.0, last.1);
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

fn timer(
    interfaces: Rc<Vec<String>>,
    interface_picker: gtk4::DropDown,
    networks: Rc<RefCell<Networks>>,
    paused: Rc<Cell<bool>>,
    elapsed: Rc<Cell<u64>>,
    timer_counter: gtk4::Label,
    download_speed: gtk4::Label,
    upload_speed: gtk4::Label,
    download_total: gtk4::Label,
    upload_total: gtk4::Label,
    samples: Rc<RefCell<Vec<(f64, f64)>>>,
    speed_graph: gtk4::DrawingArea,
) {
    gtk4::glib::timeout_add_local(Duration::from_secs(1), move || {
        if !paused.get() {
            let next = elapsed.get() + 1;
            elapsed.set(next);
            timer_counter.set_text(&format_timer(next));
        }

        if paused.get() {
            return gtk4::glib::ControlFlow::Continue;
        }

        let selected = interface_picker.selected() as usize;
        let Some(interface_name) = interfaces.get(selected) else {
            return gtk4::glib::ControlFlow::Continue;
        };

        let readings = {
            let mut networks = networks.borrow_mut();
            network::network(&mut networks)
        };

        if let Some(values) = readings.get(interface_name) {
            let (Some(download), Some(upload), Some(total_downloaded), Some(total_uploaded)) = (
                values.get("DS"),
                values.get("US"),
                values.get("AD").and_then(|value| value.parse::<u64>().ok()),
                values.get("AU").and_then(|value| value.parse::<u64>().ok()),
            ) else {
                return gtk4::glib::ControlFlow::Continue;
            };
            let (Some(download_rate), Some(upload_rate)) = (
                values.get("SD").and_then(|value| value.parse::<f64>().ok()),
                values.get("SU").and_then(|value| value.parse::<f64>().ok()),
            ) else {
                return gtk4::glib::ControlFlow::Continue;
            };

            download_speed.set_text(&format!("{download} MB/s"));
            upload_speed.set_text(&format!("{upload} MB/s"));
            download_total.set_text(&format!(
                "Total: {:.2} MB",
                total_downloaded as f64 / 1_000_000.0
            ));
            upload_total.set_text(&format!(
                "Total: {:.2} MB",
                total_uploaded as f64 / 1_000_000.0
            ));

            let mut samples = samples.borrow_mut();
            samples.push((download_rate, upload_rate));
            if samples.len() > 60 {
                samples.remove(0);
            }
            drop(samples);
            speed_graph.queue_draw();
        }

        gtk4::glib::ControlFlow::Continue
    });
}
