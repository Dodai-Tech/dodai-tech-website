//! Builds find(|t| t.id() == "dodai")he site into the repository root: `cargo run --release` from `gen/`.
//! Markup comes from `dodai-topcoat`, icons from `dodai-icons`, the banner animations from
//! `dodai-shaders` (rendered on the GPU, played back as frames).
mod charts;
use dodai_core::{themes, ButtonVariant, ControlSize, Tone};
use dodai_icons::IconId;
use dodai_shaders::{gpu::Gpu, ShaderPalette};
use dodai_topcoat::html::{button_classes, esc, icon, tag, Cx};
use dodai_core::Surface;
use std::{fs, path::Path};

const GITHUB: &str = "https://github.com/Dodai-Tech";
const MAIL: &str = "dodaidev@gmail.com";

struct Site<'a> {
    cx: Cx<'a>,
}

impl Site<'_> {
    fn ico(&self, id: IconId) -> String {
        icon(&self.cx, id, 18, false)
    }
    fn link(&self, v: ButtonVariant, s: ControlSize, href: &str, icon: Option<IconId>, text: &str) -> String {
        let i = icon.map(|i| self.ico(i)).unwrap_or_default();
        format!("<a class=\"{}\" href=\"{href}\">{i}{}</a>", button_classes(v, Some(s), false, false), esc(text))
    }
    fn chips(&self, items: &[&str]) -> String {
        let t: String = items.iter().map(|i| tag(Tone::Info, true, i)).collect();
        format!("<div class=\"chips\">{t}</div>")
    }
    fn card(&self, ic: IconId, title: &str, inner: &str) -> String {
        let head = format!("<h3>{}{}</h3>", self.ico(ic), esc(title));
        format!("<div class=\"dodai-panel\">{head}{inner}</div>")
    }
    fn nav(&self, depth: &str) -> String {
        let l = |h: &str, t: &str| format!("<a href=\"{depth}{h}\">{t}</a>");
        format!(
            "<header><a class=\"brand\" href=\"{depth}\"><img src=\"/img/logo.png\" width=\"32\" height=\"32\" alt=\"\"><b>Dodai Tech</b></a><nav>{}{}{}<a href=\"{GITHUB}\">GitHub</a>{}</nav></header>",
            l("design-system/", "Design system"), l("products/watchfaces/", "Watch faces"), l("products/instruments/", "Instruments"), l("privacy/", "Privacy")
        )
    }
    fn page(&self, dir: &str, title: &str, desc: &str, body: &str) {
        let depth = "../".repeat(dir.matches('/').count() + usize::from(!dir.is_empty()));
        let depth = if dir.is_empty() { String::new() } else { depth };
        let html = format!(
            "<!doctype html>\n<html lang=\"en\" data-theme=\"dodai\"><head>\n<meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\n<title>{title}</title>\n<meta name=\"description\" content=\"{desc}\">\n<link rel=\"icon\" href=\"/favicon.png\">\n<link rel=\"stylesheet\" href=\"/dodai.css\"><link rel=\"stylesheet\" href=\"/site.css\">\n</head>\n<body><div class=\"wrap\">{}\n<main>{body}</main>\n<footer><span>© 2026 Dodai Tech</span><span><a href=\"{GITHUB}\">GitHub</a> · <a href=\"/privacy/\">Privacy policy</a> · <a href=\"mailto:{MAIL}\">{MAIL}</a></span></footer>\n</div></body></html>\n",
            self.nav(&depth)
        );
        let p = Path::new("..").join(dir);
        fs::create_dir_all(&p).unwrap();
        fs::write(p.join("index.html"), html).unwrap();
    }
}

const COMPONENTS: &[(&str, &str, &[&str])] = &[
    ("Foundations", "Text, headings, labels, the colour palette, type scale, motion tokens, spacing and elevation, layout, frosted panels, shape tokens and the icon sets.", &["Text · Heading · Label · Mono", "Colour palette", "Type scale", "Motion tokens", "Spacing", "Elevation", "Layout", "Frosted panels", "Shape tokens", "Icon sets"]),
    ("Shape", "Roundness, borders, corner shapes and button crops: the geometry that tells the designs apart.", &["Shape gallery", "Roundness · Borders", "Corner shape", "Button crop", "Border"]),
    ("Surface effects", "Textures, frosted glass, light effects, depth and the click ripple.", &["Presets", "Component textures", "Container textures", "Frosted glass", "Light FX", "Edges · Depth", "Click ripple"]),
    ("Actions", "Buttons in every variant, icon buttons and FABs, button groups, split buttons, toolbars and snackbars.", &["Buttons · IconButton · FAB", "Button groups · Split button", "Toolbar · Snackbar"]),
    ("Data display", "Tags, chips, badges, lists, timelines, tables with sorting and virtual scroll, code and markdown, media, clocks and timers.", &["Tags · Chips · Badges", "List items", "Rating · Tooltips", "Timeline", "Tables", "Code · Markdown", "Media", "Clocks · Timers"]),
    ("Navigation", "Menus, tabs, trees, sidebars, steppers, breadcrumbs, pagination, carousels, drawers, dialogs, sheets and app bars.", &["Popover · Hover card · Context menu", "Menu · Sub-menu", "Tabs", "Tree", "Sidebar", "Stepper", "Breadcrumb", "Pagination", "Carousel", "Drawer · Dialog · Sheet", "App bar"]),
    ("Inputs", "Toggles, checkboxes, radios, sliders, knobs, text fields, search, tag and password fields, forms and date and time pickers.", &["Toggle · Checkbox · Radio", "Checkbox variants", "Toggle group · Segmented", "Slider · Range", "Knob", "Text field", "Search", "Tag field", "Password field", "Forms", "Date · Time pickers"]),
    ("Loading", "Spinners, skeletons and loading indicators.", &["Spinner · Indeterminate · Skeleton", "Loading indicator"]),
    ("Animation", "Physics-based dynamics and one-shot mixins.", &["Dynamics · physics models", "Dynamics · one-shot mixins"]),
];

const CHARTS: &[(&str, &[&str])] = &[
    ("Trends", &["Line · area", "Time axis", "Band", "Stacked area", "Stream", "Horizon", "Sparkline", "Calendar heatmap", "Control chart", "Event timeline", "Span fan", "Peak range", "Bubble timeline"]),
    ("Comparison", &["Bar", "Bullet", "Paired bars", "Lollipop · Dumbbell", "Slope", "Bump", "Radar", "Pareto", "Diverging bars"]),
    ("Part of a whole", &["Pie", "Donut", "Treemap", "Sunburst", "Icicle", "Circle packing", "Proportional squares", "Waffle", "Marimekko", "Radial bar", "100% stacked", "Shard map"]),
    ("Distribution", &["Histogram", "Box plot", "Violin", "Ridgeline", "Beeswarm", "Bubble swarm", "ECDF", "Error bars"]),
    ("Relationship", &["Scatter", "Heatmap", "Hexbin", "Parallel coordinates", "Network graph", "Time scatter"]),
    ("Flow", &["Sankey", "Split flow", "Chord", "Arc diagram", "Funnel", "Waterfall"]),
    ("Finance", &["Candlestick", "Renko", "Order book depth", "Secondary axis", "Chart stack"]),
    ("Planning", &["Gantt"]),
    ("Status", &["Slot grid", "Status strip", "Bucket bar", "Stage bars", "Gradient scale"]),
    ("Single value", &["KPI tile"]),
    ("Examples", &["Live throughput", "Network overview"]),
];

fn main() {
    let theme = themes::all().into_iter().find(|t| t.id() == "dodai").unwrap_or_else(|| panic!("themes: {:?}", themes::all().iter().map(|t| t.id()).collect::<Vec<_>>()));
    // Only the Dodai design: smaller than the all-theme stylesheet.
    let own: Vec<_> = themes::all().into_iter().filter(|t| t.meta.family == theme.meta.family).collect();
    // The page sits on the theme's own gradient wash, as the showcase does.
    let wash = dodai_topcoat::backdrop_css(&theme).map_or(String::new(), |b| format!("\nbody{{background-image:{b};background-attachment:fixed;background-size:100% 100%}}\n"));
    fs::write("../dodai.css", dodai_topcoat::multi_theme_stylesheet(&own) + &wash).unwrap();

    let dark = themes::all().into_iter().find(|t| t.id() == "dodai-dark").expect("dodai-dark theme");
    render_showcase_shaders(&dark);
    showcase_page();

    let s = Site { cx: Cx { theme: &theme, surface: Surface::INHERIT } };
    let n_charts: usize = CHARTS.iter().map(|c| c.1.len()).sum();
    let n_sections: usize = COMPONENTS.iter().map(|c| c.2.len()).sum();
    let big = |href: &str, ic, t: &str, v| s.link(v, ControlSize::Large, href, Some(ic), t);
    let prim = ButtonVariant::Primary;
    let out = ButtonVariant::Outline;

    // Pictures of the projects: a frame of one of the site's own shaders, or the watch faces.
    let faces = |pre: &str, a: &str, b: &str| format!("<div class=\"faces\"><img src=\"{pre}img/{a}.png\" alt=\"\"><img src=\"{pre}img/{b}.png\" alt=\"\"></div>");
    let badge = |tone, t: &str| tag(tone, true, t);
    let project = |href: &str, pic: String, ic: IconId, title: &str, kind: String, text: &str, links: String| format!(
        "<div class=\"dodai-panel proj\"><a class=\"pic\" href=\"{href}\">{pic}</a><h3>{}{title}</h3><div class=\"chips kind\">{kind}</div><p>{text}</p><div class=\"row\">{links}</div></div>",
        s.ico(ic));
    let open_badges = format!("{}{}", badge(Tone::Success, "Open source"), badge(Tone::Info, "Rust"));
    let open_source = [
        project("design-system/", "<img src=\"img/showcase.jpg\" alt=\"The Dodai UI showcase\">".to_string(), IconId::Palette, "Dodai UI", open_badges.clone(),
            "A design-system kit for Rust and the web with four complete designs (Dodai, Cyberpunk, Material 3 and Neo-Brutalism), each in light and dark. Tokens, 160+ components, charts, icons, motion and shaders for gpui, iced, egui, Slint, Bevy and more.",
            format!("{}{}", s.link(prim, ControlSize::Small, "design-system/showcase/", Some(IconId::Palette), "Live showcase"), s.link(out, ControlSize::Small, "design-system/", Some(IconId::OpenInNew), "Read more"))),
        project("products/watchfaces/", faces("", "wear-synthwave", "garmin-synthwave_mountains"), IconId::Watch, "Watch face development", format!("{}{}", badge(Tone::Success, "Open source"), badge(Tone::Info, "Wear OS · Garmin")),
            "How we build watch faces for Wear OS (Watch Face Format) and Garmin Connect IQ from one design: the tooling, the design rules and the always-on approach that keeps AMOLED screens safe.",
            s.link(out, ControlSize::Small, GITHUB, Some(IconId::Code), "On GitHub")),
    ].concat();
    let paid = [
        project("products/watchfaces/", faces("", "wear-synthwave", "garmin-synthwave_mountains"), IconId::Watch, "SynthWave watch face", format!("{}{}", badge(Tone::Warning, "Paid"), badge(Tone::Info, "Wear OS · Garmin")),
            "A retro outrun sunset for your wrist, with two looks, Flat and Mountains. The same face on Wear OS and Garmin, with a true always-on mode. Available through Google Play and the Connect IQ Store.",
            s.link(prim, ControlSize::Small, "products/watchfaces/", Some(IconId::Watch), "See the watch face")),
        project("products/instruments/", "<span class=\"art\">Instruments</span>".to_string(), IconId::Speed, "Dodai Instruments", format!("{}{}", badge(Tone::Warning, "Licensed"), badge(Tone::Info, "gpui")),
            "Animated dials, gauges, scopes and cockpit displays for gpui, built on the Dodai UI design system and licensed per project.",
            s.link(prim, ControlSize::Small, "products/instruments/", Some(IconId::Speed), "Learn more")),
    ].concat();
    let stack = [
        (IconId::Phone, "Android and mobile", "Native apps in Kotlin and Jetpack Compose, built for large screens, foldables and every window size.", &["Kotlin", "Jetpack Compose", "Material 3", "Gradle"][..]),
        (IconId::Watch, "Wear OS and Garmin", "Watch Face Format faces and Connect IQ faces in Monkey C, with a true always-on mode.", &["Watch Face Format", "Connect IQ", "Always-on"][..]),
        (IconId::Terminal, "Rust and the web", "Fast native tools, desktop apps and embedded firmware in Rust, and web pages from the same design tokens.", &["Rust", "ESP32", "Web / CSS"][..]),
    ];
    let stack_html: String = stack.iter().map(|(i, t, p, c)| s.card(*i, t, &format!("<p>{p}</p>{}", s.chips(c)))).collect();
    let home = format!(
        "<div class=\"hero\"><div><h1>Open source, and a few things worth paying for.</h1><p class=\"lead\">We develop open source projects: a design-system kit with four complete designs, and the know-how to build watch faces for Wear OS and Garmin. On top of that we sell finished products built with them.</p><div class=\"row\">{}{}</div></div><div class=\"pair\"><img src=\"img/wear-synthwave.png\" alt=\"SynthWave watch face, flat look\"><img src=\"img/wear-synthwave_mountains.png\" alt=\"SynthWave watch face, mountains look\"></div></div>\
<section id=\"open-source\"><h2>Open source</h2><p class=\"lead\">Free to use and read. This is what the paid products are built on.</p><div class=\"grid two\">{open_source}</div></section>\
<section id=\"products\"><h2>Products</h2><p class=\"lead\">Finished work you can buy or license. Paying for it keeps the open source side going.</p><div class=\"grid two\">{paid}</div></section>\
<section id=\"stack\"><h2>What we build with</h2><div class=\"grid narrow\">{stack_html}</div></section>\
<section id=\"about\"><h2>About</h2><p class=\"lead\">Dodai Tech is a small software studio. We design once and build natively for each platform, from Rust libraries to Wear OS and Garmin apps. No accounts, no ads, no tracking. Our code lives in the <a href=\"{GITHUB}\">Dodai-Tech GitHub organisation</a>; write to <a href=\"mailto:{MAIL}\">{MAIL}</a>.</p></section>",
        big("#open-source", IconId::Code, "Open source", prim), big("#products", IconId::Dashboard, "Products", out));
    s.page("", "Dodai Tech – Open source and products", "Dodai Tech develops open source projects, a design-system kit with four designs and watch face development, and sells watch faces and instruments built on them.",  &home);

    // Watch faces
    let gal: String = [("wear-synthwave", "Wear OS, Flat"), ("wear-synthwave_mountains", "Wear OS, Mountains"), ("garmin-synthwave", "Garmin, Flat"), ("garmin-synthwave_mountains", "Garmin, Mountains"), ("wear-synthwave_mountains-ambient", "Wear OS, always-on"), ("garmin-synthwave_mountains-ambient", "Garmin, always-on")]
        .iter().map(|(f, c)| format!("<figure><img src=\"../../img/{f}.png\" alt=\"{c}\"><figcaption>{c}</figcaption></figure>")).collect();
    let wf = format!(
        "<section><h1>Watch faces</h1><p class=\"lead\">One look on two platforms: Wear OS in Watch Face Format, and Garmin Connect IQ. Every face has an always-on variant that keeps the lit area small, so AMOLED screens stay safe.</p></section>\
<section><h2>SynthWave</h2><div class=\"stage\"><div class=\"gal\">{gal}</div></div></section>\
<section><div class=\"grid\"><div class=\"dodai-panel\">{}<h3>{}SynthWave for Wear OS</h3><p>A watch face in Watch Face Format with two looks, Flat and Mountains at dusk. Light on the battery, with a true always-on mode.</p></div><div class=\"dodai-panel\">{}<h3>{}SynthWave for Garmin</h3><p>The same watch face for Connect IQ, with an AMOLED-safe always-on mode.</p></div></div></section>",
        faces("../../", "wear-synthwave", "wear-synthwave_mountains"), s.ico(IconId::Watch), faces("../../", "garmin-synthwave", "garmin-synthwave_mountains"), s.ico(IconId::Watch));
    s.page("products/watchfaces", "Watch faces – Dodai Tech", "Watch faces for Wear OS and Garmin with matching looks and always-on modes.",  &wf);

    // Design system
    let cards = [
        (IconId::Devices, "Toolkits", "<p>A native adapter per framework, all from the same tokens.</p>".to_string(), s.chips(&["gpui", "iced", "egui", "Slint", "Bevy", "Dioxus", "Yew", "Leptos", "Freya", "ratatui", "Web (CSS)"])),
        (IconId::Palette, "Designs", "<p>Four complete designs, each with light and dark themes.</p>".into(), s.chips(&["Dodai (frosted glass)", "Cyberpunk", "Material 3", "Neo-Brutalism"])),
        (IconId::Extension, "Components", format!("<p>160+ components in {n_sections} showcase sections, from buttons to date-range pickers.</p>"), format!("<div class=\"row\">{}</div>", s.link(out, ControlSize::Small, "components/", None, "Browse components"))),
        (IconId::BarChart, "Charts", format!("<p>{n_charts} chart types for dashboards and analysis, in eleven families.</p>"), format!("<div class=\"row\">{}</div>", s.link(out, ControlSize::Small, "charts/", None, "Browse charts"))),
        (IconId::Brush, "Shaders", format!("<p>{} pixel shaders written in Rust that follow the theme palette: cityscapes, CRT and VHS monitors, painted scenes. The banner behind this page is one of them.</p>", dodai_shaders::catalog().len()), String::new()),
        (IconId::Memory, "Icons and motion", "<p>One typed icon id across icon packs, plus easing, springs, stagger and entry effects.</p>".into(), String::new()),
    ];
    let cards: String = cards.iter().map(|(i, t, p, c)| s.card(*i, t, &format!("{p}{c}"))).collect();
    s.page("design-system", "Dodai design system – Dodai Tech", "Tokens, components, themes, charts, icons and motion for Rust and the web.", 
        &format!("<section><h1>The Dodai design system</h1><p class=\"lead\">One token set, many toolkits. Colour roles, type, spacing, shape, glow and motion are defined once and resolved into components for every framework we use, so an app, a dashboard and a website feel like one product.</p><div class=\"row\">{}{}{}{}</div></section><section><iframe class=\"live\" src=\"showcase/\" title=\"The Dodai UI showcase\" loading=\"lazy\"></iframe></section><section><div class=\"grid\">{cards}</div></section>",
            s.link(prim, ControlSize::Large, "showcase/", Some(IconId::Palette), "Open the live showcase"), s.link(out, ControlSize::Large, "components/", Some(IconId::Extension), "Components"), s.link(out, ControlSize::Large, "charts/", Some(IconId::BarChart), "Charts"), s.link(out, ControlSize::Large, &format!("{GITHUB}/dodai-ui"), Some(IconId::Code), "GitHub")));

    let comp: String = COMPONENTS.iter().map(|(h, d, c)| format!("<section><h2>{h}</h2><p class=\"lead\">{d}</p>{}</section>", s.chips(c))).collect();
    s.page("design-system/components", "Components – Dodai Tech", "Every component section of the Dodai design system.", 
        &format!("<section><h1>Components</h1><p class=\"lead\">The component sections of the Dodai design system, by category. Each one is shown in every design and theme in the live showcase.</p><div class=\"row\">{}{}</div></section>{comp}",
            s.link(out, ControlSize::Medium, "../", Some(IconId::Palette), "Design system"), s.link(out, ControlSize::Medium, "../charts/", Some(IconId::BarChart), "Charts")));

    let gal2 = charts::gallery();
    fs::write("../charts.css", charts::CSS).unwrap();
    let ch: String = CHARTS.iter().map(|(h, c)| format!("<section><h2>{h}</h2>{}</section>", s.chips(c))).collect();
    s.page("design-system/charts", "Charts – Dodai Tech", "The chart types of the Dodai design system.", 
        &format!("<section><h1>Charts</h1><p class=\"lead\">{n_charts} chart types in eleven families. They draw from the active theme, so every chart follows the design and light or dark mode, and they animate with live data.</p><div class=\"row\">{}{}</div></section>{gal2}{ch}",
            s.link(out, ControlSize::Medium, "../", Some(IconId::Palette), "Design system"), s.link(out, ControlSize::Medium, "../components/", Some(IconId::Extension), "Components")));

    let sets = [
        (IconId::Speed, "Automotive", "Dashes, pods, gauge faces in twelve looks, tachometer, speedometers, tyre pressure, gear selector and every drivetrain."),
        (IconId::Dashboard, "Aviation", "Attitude indicator, heading and vertical tapes, sonar waterfall and countermeasures."),
        (IconId::Layers, "Defence", "Tactical plot, track list, threat rosette, stores panel, mission timer and radar scope."),
        (IconId::Memory, "Medical", "Vitals tiles and the ECG trace."),
        (IconId::Extension, "Industrial", "Process-plant and machine symbols."),
        (IconId::BarChart, "Always included", "Dials, gauges, segment displays, annunciators and scopes. Limits and alarms colour the reading itself."),
    ];
    let sets: String = sets.iter().map(|(i, t, p)| s.card(*i, t, &format!("<p>{p}</p>"))).collect();
    s.page("products/instruments", "Dodai Instruments – Dodai Tech", "Commercial gauges, dials and displays for Rust and gpui.", 
        &format!("<section><h1>Dodai Instruments</h1><p class=\"lead\">Animated dials, gauges, scopes and cockpit displays for gpui, built on the Dodai design system and licensed per project.</p><div class=\"row\">{}</div></section><section><div class=\"grid\">{sets}</div></section><section><p>Each industry set builds only its own instruments, so a vehicle-only app compiles no medical or defence code. Colours come from the active Dodai theme.</p></section>",
            s.link(prim, ControlSize::Large, &format!("mailto:{MAIL}?subject=Dodai%20Instruments"), Some(IconId::Mail), "Ask for a licence")));
}

/// The shader pictures of the showcase page: every generator alone and every preset, one still
/// each (the page cannot run the GPU renderer). `host.js` picks by preset id or first shader id.
fn render_showcase_shaders(theme: &dodai_core::Theme) {
    use dodai_shaders::{catalog, presets, Kind};
    let dir = Path::new("../design-system/showcase/shaders");
    fs::create_dir_all(dir).unwrap();
    let mut gpu = Gpu::new().expect("no GPU");
    let mut jobs: Vec<(String, Vec<&'static str>, ShaderPalette)> = catalog()
        .iter()
        .filter(|s| s.kind == Kind::Generator)
        .map(|s| (s.id.to_string(), vec![s.id], ShaderPalette::from_theme(theme)))
        .collect();
    jobs.extend(presets::all().map(|p| (p.id.to_string(), p.chain.to_vec(), (p.palette)())));
    for (name, chain, pal) in jobs {
        // 12 frames, played forward and back, so the loop has no seam.
        let mut frames = Vec::new();
        for f in 0..12 {
            let Ok(bgra) = gpu
                .render(&chain, 480, 270, 1.0 + f as f32 * 0.15, 1.0, &pal)
                .map(<[u8]>::to_vec)
            else {
                break; // a shader the adapter cannot run: the page leaves its picture out
            };
            let rgb: Vec<u8> = bgra.chunks_exact(4).flat_map(|p| [p[2], p[1], p[0]]).collect();
            let mut buf = Vec::new();
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 80)
                .encode(&rgb, 480, 270, image::ExtendedColorType::Rgb8)
                .unwrap();
            frames.push(buf);
        }
        if frames.len() < 12 {
            continue;
        }
        fs::write(dir.join(format!("{name}.jpg")), &frames[0]).unwrap();
        let tmp = std::env::temp_dir().join("dodai-site-frames");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        for (n, i) in (0..12).chain((1..11).rev()).enumerate() {
            fs::write(tmp.join(format!("{n:03}.jpg")), &frames[i]).unwrap();
        }
        let ok = std::process::Command::new("ffmpeg")
            .args(["-v", "error", "-y", "-framerate", "10", "-i"])
            .arg(tmp.join("%03d.jpg"))
            .args(["-c:v", "libx264", "-crf", "30", "-pix_fmt", "yuv420p", "-movflags", "+faststart"])
            .arg(dir.join(format!("{name}.mp4")))
            .status()
            .expect("ffmpeg is needed to make the shader loops");
        assert!(ok.success(), "ffmpeg failed for {name}");
    }
}

fn showcase_page() {
    let html = "<!doctype html>\n<html lang=\"en\" data-theme=\"dodai\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>Dodai UI showcase – Dodai Tech</title><link rel=\"icon\" href=\"/favicon.png\"><style>html,body{margin:0;height:100%}body{background:var(--dodai-background);color:var(--dodai-text);font-family:var(--dodai-font-body);font-size:var(--dodai-size-md);line-height:1.5}</style></head><body><noscript>The showcase needs JavaScript and WebAssembly.</noscript><script src=\"host.js\"></script></body></html>\n";
    fs::write("../design-system/showcase/index.html", html).unwrap();
}
