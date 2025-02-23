use bevy::prelude::*;

#[derive(Debug, Clone, Reflect)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, Clone, Reflect)]
pub enum Axis {
    Horizontal,
    Vertical,
    HorizontalAndVertical,
}

impl From<&komorebi_client::Layout> for Layout {
    fn from(layout: &komorebi_client::Layout) -> Self {
        match layout {
            komorebi_client::Layout::Default(dl) => Layout::Default(dl.into()),
            komorebi_client::Layout::Custom(cl) => Layout::Custom(cl.into()),
        }
    }
}

impl From<&komorebi_client::DefaultLayout> for DefaultLayout {
    fn from(layout: &komorebi_client::DefaultLayout) -> Self {
        match layout {
            komorebi_client::DefaultLayout::BSP => Self::BSP,
            komorebi_client::DefaultLayout::Columns => Self::Columns,
            komorebi_client::DefaultLayout::Rows => Self::Rows,
            komorebi_client::DefaultLayout::VerticalStack => Self::VerticalStack,
            komorebi_client::DefaultLayout::HorizontalStack => Self::HorizontalStack,
            komorebi_client::DefaultLayout::UltrawideVerticalStack => Self::UltrawideVerticalStack,
            komorebi_client::DefaultLayout::Grid => Self::Grid,
            komorebi_client::DefaultLayout::RightMainVerticalStack => Self::RightMainVerticalStack,
        }
    }
}

impl From<&komorebi_client::CustomLayout> for CustomLayout {
    fn from(cl: &komorebi_client::CustomLayout) -> Self {
        CustomLayout(
            cl.iter()
                .map(|c| c.into())
                .collect()
        )
    }
}

impl From<&komorebi_client::Column> for Column {
    fn from(col: &komorebi_client::Column) -> Self {
        match col {
            komorebi_client::Column::Primary(cw) => 
                Column::Primary(cw.as_ref().map(|w| w.into())),
            komorebi_client::Column::Secondary(csc) => 
                Column::Secondary(csc.as_ref().map(|sc| sc.into())),
            komorebi_client::Column::Tertiary(cs) => 
                Column::Tertiary(cs.into()),
        }
    }
}

impl From<&komorebi_client::ColumnWidth> for ColumnWidth {
    fn from(cw: &komorebi_client::ColumnWidth) -> Self {
        match cw {
            komorebi_client::ColumnWidth::WidthPercentage(p) => 
                ColumnWidth::WidthPercentage(*p),
        }
    }
}

impl From<&komorebi_client::ColumnSplit> for ColumnSplit {
    fn from(cs: &komorebi_client::ColumnSplit) -> Self {
        match cs {
            komorebi_client::ColumnSplit::Horizontal => ColumnSplit::Horizontal,
            komorebi_client::ColumnSplit::Vertical => ColumnSplit::Vertical,
        }
    }
}

impl From<&komorebi_client::ColumnSplitWithCapacity> for ColumnSplitWithCapacity {
    fn from(csc: &komorebi_client::ColumnSplitWithCapacity) -> Self {
        match csc {
            komorebi_client::ColumnSplitWithCapacity::Horizontal(cap) => 
                ColumnSplitWithCapacity::Horizontal(*cap),
            komorebi_client::ColumnSplitWithCapacity::Vertical(cap) => 
                ColumnSplitWithCapacity::Vertical(*cap),
        }
    }
}

impl From<&komorebi_client::Rect> for Rect {
    fn from(r: &komorebi_client::Rect) -> Self {
        Self {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}

#[derive(Component, Reflect, Debug, Clone)]
pub enum Layout {
    Default(DefaultLayout),
    Custom(CustomLayout),
}

#[derive(Component, Reflect, Debug, Clone)]
pub enum DefaultLayout {
    BSP,
    Columns,
    Rows,
    VerticalStack,
    HorizontalStack,
    UltrawideVerticalStack,
    Grid,
    RightMainVerticalStack,
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct CustomLayout(pub Vec<Column>);

#[derive(Component, Reflect, Debug, Clone)]
pub enum Column {
    Primary(Option<ColumnWidth>),
    Secondary(Option<ColumnSplitWithCapacity>),
    Tertiary(ColumnSplit),
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
pub enum ColumnWidth {
    WidthPercentage(f32),
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
pub enum ColumnSplit {
    Horizontal,
    Vertical,
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
pub enum ColumnSplitWithCapacity {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Component, Reflect)]
pub struct Monitor {
    pub id: isize,
    pub name: String,
    pub device: String,
    pub device_id: String,
    pub physical_size: Rect,
    pub work_area_size: Rect,
    pub work_area_offset: Option<Rect>,
    pub window_based_work_area_offset: Option<Rect>,
    pub window_based_work_area_offset_limit: isize,
}

#[derive(Component, Reflect)]
pub struct Workspace {
    pub name: Option<String>,
    pub layout: Layout,
    pub monocle_container_restore_idx: Option<usize>,
    pub maximized_window_restore_idx: Option<usize>,
    pub floating_windows: Vec<Entity>,
    pub layout_rules: Vec<(usize, Layout)>,
    pub layout_flip: Option<Axis>,
    pub workspace_padding: Option<i32>,
    pub container_padding: Option<i32>,
    pub latest_layout: Vec<Rect>,
    pub resize_dimensions: Vec<Option<Rect>>,
    pub tile: bool,
    pub apply_window_based_work_area_offset: bool,
    pub float_override: Option<bool>,
}

#[derive(Component, Reflect)]
pub struct Container {
    pub id: String,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq)]
pub struct Window {
    pub hwnd: isize,
}

#[derive(Component, Reflect)]
pub struct MonocleContainer(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct FloatingWindow;

#[derive(Component, Reflect)]
pub struct FocusedWindow(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct FocusedContainer(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct LastFocusedContainer(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct FocusedWorkspace(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct LastFocusedWorkspace(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct MaximizedWindow(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct Focused(pub i32);
