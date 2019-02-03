use crate::geom::{FMargins, FRect, Margins, Size};
use crate::render::frame;
use crate::ui::view::Base;
use crate::ui::view::{self, HasMargins, HasPadding, MeasureSpec};
use crate::ui::View;

use std::cell::Cell;
use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub enum Orientation {
    Horizontal = 0,
    Vertical = 1,
}

impl Orientation {
    pub fn ortho(self) -> Orientation {
        match self {
            Orientation::Horizontal => Orientation::Vertical,
            Orientation::Vertical => Orientation::Horizontal,
        }
    }
}

impl<T: Copy + Add<Output = T>> Margins<T> {
    /// Get the Margin along the specified orientation
    fn along(self, orientation: Orientation) -> T {
        match orientation {
            Orientation::Horizontal => self.horizontal(),
            Orientation::Vertical => self.vertical(),
        }
    }
    /// Get top or left margin depending on specified orientation
    fn along_before(self, orientation: Orientation) -> T {
        match orientation {
            Orientation::Horizontal => self.left(),
            Orientation::Vertical => self.top(),
        }
    }
    /// Get bottom or right margin depending on specified orientation
    fn along_after(self, orientation: Orientation) -> T {
        match orientation {
            Orientation::Horizontal => self.right(),
            Orientation::Vertical => self.bottom(),
        }
    }
}

impl<T: Copy + Add<Output = T>> Size<T> {
    /// Get the Margin along the specified orientation
    fn along(self, orientation: Orientation) -> T {
        match orientation {
            Orientation::Horizontal => self.width(),
            Orientation::Vertical => self.height(),
        }
    }
}

mod grav {
    /// Content should be centered within container
    pub const CENTER: u8 = 0x01;
    /// State how left and top edge should be placed
    pub const PULL_BEFORE: u8 = 0x02;
    /// State how right and bottom edge should be placed
    pub const PULL_AFTER: u8 = 0x04;
    /// Whether the right and bottom edge should be clipped to the container
    pub const CLIP: u8 = 0x08;
    /// Mask for the gravity of one axis
    pub const MASK: u8 = 0x0f;
    /// Shift value to apply to get the the horizontal gravity
    pub const SHIFT_HOR: u8 = 0;
    /// Shift value to apply to get the the vertical gravity
    pub const SHIFT_VER: u8 = 4;
}

bitflags! {
    /// Treats gravity in one axis
    pub struct AxisGravity: u8 {
        const CENTER        = grav::CENTER;
        const PULL_BEFORE   = grav::PULL_BEFORE;
        const PULL_AFTER    = grav::PULL_AFTER;
        const CLIP          = grav::CLIP;

        const FILL          = grav::PULL_BEFORE | grav::PULL_AFTER;
    }
}

bitflags! {
    pub struct Gravity: u8 {
        /// whether left edge should fit the parent
        const LEFT              = grav::PULL_BEFORE << grav::SHIFT_HOR;
        /// whether top edge should fit the parent
        const TOP               = grav::PULL_BEFORE << grav::SHIFT_VER;
        /// whether right edge should fit the parent
        const RIGHT             = grav::PULL_AFTER << grav::SHIFT_HOR;
        /// whether bottom edge should fit the parent
        const BOTTOM            = grav::PULL_AFTER << grav::SHIFT_VER;

        /// whether top and left edge should fit the parent
        const TOP_LEFT          = Self::LEFT.bits | Self::TOP.bits;
        /// whether top and right edge should fit the parent
        const TOP_RIGHT         = Self::RIGHT.bits | Self::TOP.bits;
        /// whether bottom and left edge should fit the parent
        const BOTTOM_LEFT       = Self::LEFT.bits | Self::BOTTOM.bits;
        /// whether bottom and right edge should fit the parent
        const BOTTOM_RIGHT      = Self::RIGHT.bits | Self::BOTTOM.bits;

        /// whether the child should be centered horizontally
        const CENTER_HOR        = grav::CENTER << grav::SHIFT_HOR;
        /// whether the child should be centered vertically
        const CENTER_VER        = grav::CENTER << grav::SHIFT_VER;
        /// whether the child should be centered on both axis
        const CENTER            = Self::CENTER_HOR.bits | Self::CENTER_VER.bits;

        /// whether the child should be filled horizontally
        const FILL_HOR          = Self::LEFT.bits | Self::RIGHT.bits;
        /// whether the child should be filled vertically
        const FILL_VER          = Self::TOP.bits | Self::BOTTOM.bits;
        /// whether the child should be filled on both axis
        const FILL              = Self::FILL_HOR.bits | Self::FILL_VER.bits;

        /// whether the child should be clipped horizontally
        const CLIP_HOR          = grav::CLIP << grav::SHIFT_HOR;
        /// whether the child should be clipped vertically
        const CLIP_VER          = grav::CLIP << grav::SHIFT_VER;
        /// whether the child should be clipped on both axis
        const CLIP              = Self::CLIP_HOR.bits | Self::CLIP_VER.bits;
    }
}

impl Gravity {
    /// Get the horizontal gravity
    fn horizontal(self) -> AxisGravity {
        AxisGravity {
            bits: (self.bits >> grav::SHIFT_HOR) & grav::MASK,
        }
    }
    /// Get the vertical gravity
    fn vertical(self) -> AxisGravity {
        AxisGravity {
            bits: (self.bits >> grav::SHIFT_VER) & grav::MASK,
        }
    }
    /// Get the gravity along specified orientation
    fn along(self, orientation: Orientation) -> AxisGravity {
        match orientation {
            Orientation::Horizontal => self.horizontal(),
            Orientation::Vertical => self.vertical(),
        }
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Gravity::TOP_LEFT
    }
}

/// The size in a Layout, a scalar or special tokens "MatchParent" or "WrapContent"
#[derive(Clone, Copy, Debug)]
pub enum LayoutSize {
    Scalar(f32),
    MatchParent,
    WrapContent,
}

#[derive(Debug)]
pub struct LinearLayout {
    common: view::Common,
    orientation: Orientation,
    views: Vec<Box<dyn View>>,
    total_length: Cell<f32>,
    gravity: Gravity,
    spacing: f32,
}

impl LinearLayout {
    pub fn new(orientation: Orientation) -> LinearLayout {
        LinearLayout {
            common: view::Common::default(),
            orientation,
            views: Vec::new(),
            total_length: Cell::new(0f32),
            gravity: Default::default(),
            spacing: 0f32,
        }
    }

    pub fn new_horizontal() -> LinearLayout {
        Self::new(Orientation::Horizontal)
    }

    pub fn new_vertical() -> LinearLayout {
        Self::new(Orientation::Vertical)
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn add_view(&mut self, view: Box<dyn View>) {
        self.views.push(view)
    }

    pub fn gravity(&self) -> Gravity {
        self.gravity
    }

    pub fn set_gravity(&mut self, gravity: Gravity) {
        self.gravity = gravity;
    }

    pub fn spacing(&self) -> f32 {
        self.spacing
    }

    pub fn set_spacing(&mut self, spacing: f32) {
        self.spacing = spacing;
    }
}

impl LinearLayout {
    fn measure_child(
        padding: FMargins,
        view: &mut dyn View,
        parent_specs: [MeasureSpec; 2],
        used_space: [f32; 2],
    ) {
        let ws = child_measure_spec(
            parent_specs[0],
            padding.horizontal() + used_space[0],
            LayoutSize::WrapContent,
        );
        let hs = child_measure_spec(
            parent_specs[1],
            padding.vertical() + used_space[1],
            LayoutSize::WrapContent,
        );
        view.measure([ws, hs]);
    }
}

impl View for LinearLayout {}

impl view::Measure for LinearLayout {
    fn measure(&mut self, specs: [MeasureSpec; 2]) {
        let mut total = [0f32; 2];
        let mut largest_ortho = 0f32;
        //let mut total_weight = 0f32;
        let ind = self.orientation as usize;
        let ind_ortho = self.orientation.ortho() as usize;

        let padding = self.padding();

        for view in &mut self.views {
            Self::measure_child(padding, &mut **view, specs, total);
            let m: [f32; 2] = From::from(view.measurement());
            total[ind] += m[ind];
            largest_ortho =
                largest_ortho.max(m[ind_ortho] + view.margins().along(self.orientation.ortho()));
            // TODO weight
        }
        total[ind] += self.padding().along(self.orientation);

        let mut too_small = [false, false];
        //let mut final_size = resolve_size(total[ind], specs[ind], &mut too_small[ind]);
        //let mut remain_excess = final_size - total[ind];

        // TODO distribute remain_excess according weight

        largest_ortho += self.padding().along(self.orientation.ortho());
        total[ind_ortho] = largest_ortho;
        self.set_measurement(Size(
            resolve_size(total[0], specs[0], &mut too_small[0]),
            resolve_size(total[1], specs[1], &mut too_small[1]),
        ));
        if too_small[0] || too_small[1] {
            println!("layout too small!");
        }
        self.total_length.set(total[ind]);
    }
}

impl view::Layout for LinearLayout {
    fn layout(&mut self, rect: FRect) {
        let orientation = self.orientation;
        let ortho = self.orientation.ortho();
        let padding = self.padding();
        let margins = self.margins();

        let mut child_before = match self.gravity.along(self.orientation) {
            AxisGravity::PULL_AFTER => {
                padding.along(self.orientation) + rect.size().along(self.orientation)
                    - self.total_length.get()
            }
            AxisGravity::CENTER => {
                padding.along(self.orientation)
                    + (rect.size().along(self.orientation) - self.total_length.get()) / 2f32
            }
            _ => padding.along(self.orientation),
        };

        let child_ortho_after = rect.size().along(ortho) - padding.along_after(ortho);
        let child_ortho_space = child_ortho_after - padding.along_before(ortho);
        let mut first = true;

        for view in &mut self.views {
            // TODO: child margins
            let mes = view.measurement();
            let child_ortho_before = match self.gravity.along(ortho) {
                AxisGravity::PULL_AFTER => {
                    child_ortho_after - mes.along(ortho) // - child_margins.along(ortho)
                }
                AxisGravity::CENTER => {
                    padding.along_before(ortho)
                        + (child_ortho_space - mes.along(ortho)/* - child_margins.along(ortho) */)
                            / 2f32
                }
                _ => padding.along_before(ortho),
            };

            if first {
                child_before += self.spacing;
                first = false;
            }
            let mut point = [0f32; 2];
            point[orientation as usize] = child_before + margins.along_before(orientation);
            point[ortho as usize] = child_ortho_before + margins.along_before(ortho);

            view.layout(FRect::new_s(point[0], point[1], mes));

            child_before += mes.along(self.orientation);
        }
    }
}

impl view::FrameRender for LinearLayout {
    fn frame_render(&self) -> Option<frame::Node> {
        None
    }
}

impl view::SliceParent for LinearLayout {
    fn children_slice(&self) -> &[Box<dyn View>] {
        &self.views
    }
    fn children_slice_mut(&mut self) -> &mut [Box<dyn View>] {
        &mut self.views
    }
}

impl view::Base for LinearLayout {
    type Style = ();
    type State = ();

    fn common(&self) -> &view::Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut view::Common {
        &mut self.common
    }
}

/// Provide the measure spec to be given to a child
/// # Arguments
///     * `parent_spec`         - the measure spec of the parent
///     * `removed`             - how much has been consumed so far from the parent space
///     * `child_layout_size`   - the child size given in layout params
pub fn child_measure_spec(
    parent_spec: MeasureSpec,
    removed: f32,
    child_layout_size: LayoutSize,
) -> MeasureSpec {
    match child_layout_size {
        LayoutSize::Scalar(val) => MeasureSpec::Exactly(val),
        _ => match parent_spec {
            MeasureSpec::Exactly(size) => {
                let size = 0f32.max(size - removed);
                match child_layout_size {
                    LayoutSize::WrapContent => MeasureSpec::AtMost(size),
                    LayoutSize::MatchParent => MeasureSpec::Exactly(size),
                    _ => panic!(),
                }
            }
            MeasureSpec::AtMost(size) => MeasureSpec::AtMost(0f32.max(size - removed)),
            MeasureSpec::Unspecified => MeasureSpec::Unspecified,
        },
    }
}

/// Reconciliate a measure spec and children dimensions.
/// This will give the final dimension to be shared amoung the children.
/// `too_small` will be set to true if size is bigger than the spec size, false otherwise.
pub fn resolve_size(size: f32, spec: MeasureSpec, too_small: &mut bool) -> f32 {
    *too_small = false;
    match spec {
        MeasureSpec::AtMost(at_most) => {
            if size > at_most {
                *too_small = true;
                at_most
            } else {
                size
            }
        }
        MeasureSpec::Exactly(exactly) => exactly,
        MeasureSpec::Unspecified => size,
    }
}
