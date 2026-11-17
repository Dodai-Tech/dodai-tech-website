#!/usr/bin/env nu
# Generates the design-system subpages: nu build.nu
let nav = '<header><a class="brand" href="/"><b>Dodai Tech</b></a><nav><a href="/design-system/">Design system</a><a href="/design-system/components/">Components</a><a href="/design-system/charts/">Charts</a><a href="/#products">Products</a><a href="https://github.com/Dodai-Tech">GitHub</a><a href="/privacy/">Privacy</a></nav></header>'
let foot = '<footer><span>© 2026 Dodai Tech</span><span><a href="https://github.com/Dodai-Tech">GitHub</a> · <a href="/privacy/">Privacy policy</a> · <a href="mailto:dodaidev@gmail.com">dodaidev@gmail.com</a></span></footer>'

def page [dir: string, title: string, desc: string, body: string] {
    mkdir $dir
    $"<!doctype html>
<html lang=\"en\" data-theme=\"dodai-dark\"><head>
<meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">
<title>($title) – Dodai Tech</title>
<meta name=\"description\" content=\"($desc)\">
<link rel=\"icon\" href=\"/favicon.png\">
<link rel=\"stylesheet\" href=\"/dodai.css\"><link rel=\"stylesheet\" href=\"/site.css\">
</head>
<body><div class=\"wrap\">($nav)
<main>($body)</main>
($foot)
</div></body></html>\n" | save --force $"($dir)/index.html"
}

def chips [items: list<string>] { $items | each {|i| $"<span class=\"chip\">($i)</span>" } | str join "" }
def panel [h: string, body: string] { $"<div class=\"dodai-panel\"><h3>($h)</h3>($body)</div>" }

let components = [
  [Foundations "Text, headings, labels, the colour palette, type scale, motion tokens, spacing and elevation, layout, frosted panels, shape tokens and the icon sets." ["Text · Heading · Label · Mono" "Colour palette" "Type scale" "Motion tokens" "Spacing · Elevation" "Layout · Fluid scales" "Frosted panels" "Shape tokens" "Icons"]]
  [Shape "Roundness, borders, corner shapes and button crops: the geometry that tells the designs apart." ["Shape gallery" "Roundness · Borders" "Corner shape" "Button crop" "Border"]]
  ["Surface effects" "Textures, frosted glass, light effects, depth and the click ripple." ["Presets" "Component textures" "Container textures" "Frosted glass" "Light FX" "Edges · Depth" "Click ripple" "Signature"]]
  [Actions "Buttons in every variant, icon buttons and FABs, button groups, split buttons, toolbars and snackbars." ["Buttons · IconButton · FAB" "Button groups · Split button" "Toolbar · Snackbar"]]
  ["Data display" "Tags, chips, badges, lists, timelines, tables with sorting and virtual scroll, code and markdown, media, clocks and timers." ["Tags · Chips · Badges" "List items" "Rating · Tooltips" "List · Divider" "Timeline · Empty state" "Rich tooltip" "Data table" "Code block · Markdown · rich text" "Image · media · Avatar group" "Circular progress · Banner" "Clock · Timer" "Stopwatch · Tap tempo" "Scroll area · styled scrollbar"]]
  [Navigation "Menus, tabs, trees, sidebars, steppers, breadcrumbs, pagination, carousels, drawers, dialogs, sheets and app bars." ["Popover · Hover card · Context menu" "Menu · Sub-menu" "Tabs" "Tree" "Axis gizmo" "Collapsible sidebar" "Segmented · Breadcrumb · Pagination · Stepper" "Carousel" "Snap stack" "Navigation drawer" "Dialog · Bottom sheet · Side sheet" "App bars"]]
  [Inputs "Toggles, checkboxes, radios, sliders, knobs, text fields, search, tag and password fields, forms and date and time pickers." ["Toggle · Checkbox · Radio" "Checkbox variants" "Toggle group · Select all" "Slider · Progress" "Stepped slider" "Range slider" "Knob · Vertical sliders" "Input field" "Search view · SearchBar" "Tag field" "Password · One-time code" "Form field with validation" "Date picker · Time picker" "Date range picker" "Date-time range picker"]]
  [Loading "Spinners, skeletons and loading indicators." ["Spinner · Indeterminate · Skeleton" "Loading indicator"]]
  [Animation "Physics-based dynamics and one-shot mixins." ["Dynamics · physics models" "Dynamics · one-shot mixins"]]
]

let charts = [
  [Trends ["Line · area" "Time axis" "Band" "Stacked area" "Stream" "Horizon" "Sparkline" "Calendar heatmap" "Control chart" "Event timeline" "Span fan" "Peak range" "Bubble timeline"]]
  [Comparison ["Bar" "Bullet" "Paired bars" "Lollipop · Dumbbell" "Slope" "Bump" "Radar" "Pareto" "Diverging bars"]]
  ["Part of a whole" ["Pie" "Donut" "Treemap" "Sunburst" "Icicle" "Circle packing" "Proportional squares" "Waffle" "Marimekko" "Radial bar" "100% stacked" "Shard map"]]
  [Distribution ["Histogram" "Box plot" "Violin" "Ridgeline" "Beeswarm" "Bubble swarm" "ECDF" "Error bars"]]
  [Relationship ["Scatter" "Heatmap" "Hexbin" "Parallel coordinates" "Network graph" "Time scatter"]]
  [Flow ["Sankey" "Split flow" "Chord" "Arc diagram" "Funnel" "Waterfall"]]
  [Finance ["Candlestick" "Renko" "Order book depth" "Secondary axis" "Chart stack"]]
  [Planning ["Gantt"]]
  [Status ["Slot grid" "Status strip" "Bucket bar" "Stage bars" "Gradient scale"]]
  ["Single value" ["KPI tile"]]
  [Examples ["Live throughput" "Network overview"]]
]
let n_charts = $charts | each {|c| $c.1 | length } | math sum
let n_sections = $components | each {|c| $c.2 | length } | math sum

# Overview
let toolkits = [gpui iced egui Slint Bevy Dioxus Yew Leptos Freya ratatui "Web (CSS)"]
let designs = ["Dodai (frosted glass)" "Cyberpunk" "Material 3" "Neo-Brutalism"]
page design-system "Dodai design system" "Tokens, components, themes, charts, icons and motion for Rust and the web." $"
<section><h1>The Dodai design system</h1>
<p class=\"lead\">One token set, many toolkits. Colour roles, type, spacing, shape, glow and motion are defined once and resolved into components for every framework we use, so an app, a dashboard and a web page look like the same product.</p>
<div class=\"row\"><a class=\"dodai-btn dodai-btn--primary dodai-btn--lg\" href=\"components/\">Components</a><a class=\"dodai-btn dodai-btn--outline dodai-btn--lg\" href=\"charts/\">Charts</a><a class=\"dodai-btn dodai-btn--outline dodai-btn--lg\" href=\"https://github.com/Dodai-Tech/dodai-ui\">Dodai UI on GitHub</a></div></section>
<section><div class=\"grid\">
(panel 'Toolkits' $'<p>A native adapter per framework, all from the same tokens.</p><div class="chips">(chips $toolkits)</div>')
(panel 'Designs' $'<p>Four complete designs, each with light and dark themes.</p><div class="chips">(chips $designs)</div>')
(panel 'Components' $'<p>160+ components in ($n_sections) showcase sections, from buttons to date-range pickers.</p><div class="row"><a href="components/">Browse components</a></div>')
(panel 'Charts' $'<p>($n_charts) chart types for dashboards and analysis, in eleven families.</p><div class="row"><a href="charts/">Browse charts</a></div>')
(panel 'Shaders' '<p>196 pixel shaders written in Rust that follow the theme palette: cityscapes, CRT and VHS monitors, painted scenes.</p>')
(panel 'Icons and motion' '<p>One typed icon id across icon packs, plus easing, springs, stagger and entry effects.</p>')
</div></section>"

# Components
let comp_body = ($components | each {|c|
  $"<section><h2>($c.0)</h2><p class=\"lead\">($c.1)</p><div class=\"chips\">(chips $c.2)</div></section>" } | str join "")
page design-system/components "Components" "Every component section of the Dodai design system." $"<section><h1>Components</h1><p class=\"lead\">The component sections of the Dodai design system, by category. Each one is shown in every design and theme in the live showcase.</p><div class=\"row\"><a class=\"dodai-btn dodai-btn--outline\" href=\"../\">Design system</a><a class=\"dodai-btn dodai-btn--outline\" href=\"../charts/\">Charts</a></div></section>($comp_body)"

# Charts
let chart_body = ($charts | each {|c|
  $"<section><h2>($c.0)</h2><div class=\"chips\">(chips $c.1)</div></section>" } | str join "")
page design-system/charts "Charts" "The chart types of the Dodai design system." $"<section><h1>Charts</h1><p class=\"lead\">($n_charts) chart types in eleven families. They draw from the active theme, so every chart follows the design and light or dark mode, and they animate with live data.</p><div class=\"row\"><a class=\"dodai-btn dodai-btn--outline\" href=\"../\">Design system</a><a class=\"dodai-btn dodai-btn--outline\" href=\"../components/\">Components</a></div></section>($chart_body)"

# Instruments product
page products/instruments "Dodai Instruments" "Commercial gauges, dials and displays for Rust and gpui." $"
<section><h1>Dodai Instruments</h1>
<p class=\"lead\">Animated dials, gauges, scopes and cockpit displays for gpui, built on the Dodai design system and licensed per project.</p>
<div class=\"row\"><a class=\"dodai-btn dodai-btn--primary dodai-btn--lg\" href=\"mailto:dodaidev@gmail.com?subject=Dodai%20Instruments\">Ask for a licence</a></div></section>
<section><div class=\"grid\">
(panel 'Automotive' '<p>Dashes, pods, gauge faces in twelve looks, tachometer, speedometers, tyre pressure, gear selector and every drivetrain.</p>')
(panel 'Aviation' '<p>Attitude indicator, heading and vertical tapes, sonar waterfall and countermeasures.</p>')
(panel 'Defence' '<p>Tactical plot, track list, threat rosette, stores panel, mission timer and radar scope.</p>')
(panel 'Medical' '<p>Vitals tiles and the ECG trace.</p>')
(panel 'Industrial' '<p>Process-plant and machine symbols.</p>')
(panel 'Always included' '<p>Dials, gauges, segment displays, annunciators and scopes. Limits and alarms colour the reading itself.</p>')
</div></section>
<section><p>Each industry set builds only its own instruments, so a vehicle-only app compiles no medical or defence code. Colours come from the active Dodai theme.</p></section>"
