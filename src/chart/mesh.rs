use std::fmt::Debug;
use std::marker::PhantomData;

use super::builder::LabelAreaPosition;
use super::context::ChartContext;
use crate::coord::{MeshLine, Ranged, RangedCoord};
use crate::drawing::backend::DrawingBackend;
use crate::drawing::DrawingAreaErrorKind;
use crate::style::{
    AsRelative, Color, FontDesc, IntoTextStyle, RGBColor, ShapeStyle, SizeDesc, TextStyle,
};

/// The style used to describe the mesh for a secondary coordinate system.
pub struct SecondaryMeshStyle<'a, 'b, X: Ranged, Y: Ranged, DB: DrawingBackend> {
    style: MeshStyle<'a, 'b, X, Y, DB>,
}

impl<'a, 'b, X: Ranged, Y: Ranged, DB: DrawingBackend> SecondaryMeshStyle<'a, 'b, X, Y, DB>
where
    X::ValueType: Debug,
    Y::ValueType: Debug,
{
    pub(super) fn new(target: &'b mut ChartContext<'a, DB, RangedCoord<X, Y>>) -> Self {
        let mut style = target.configure_mesh();
        style.draw_x_mesh = false;
        style.draw_y_mesh = false;
        Self { style }
    }

    /// Set the style definition for the axis
    /// - `style`: The style for the axis
    pub fn axis_style<T: Into<ShapeStyle>>(&mut self, style: T) -> &mut Self {
        self.style.axis_style(style);
        self
    }

    /// The offset of x labels. This is used when we want to place the label in the middle of
    /// the grid. This is useful if we are drawing a histogram
    /// - `value`: The offset in pixel
    pub fn x_label_offset<S: SizeDesc>(&mut self, value: S) -> &mut Self {
        self.style.x_label_offset(value);
        self
    }

    /// The offset of y labels. This is used when we want to place the label in the middle of
    /// the grid. This is useful if we are drawing a histogram
    /// - `value`: The offset in pixel
    pub fn y_label_offset<S: SizeDesc>(&mut self, value: S) -> &mut Self {
        self.style.y_label_offset(value);
        self
    }

    /// Set how many labels for the X axis at most
    /// - `value`: The maximum desired number of labels in the X axis
    pub fn x_labels(&mut self, value: usize) -> &mut Self {
        self.style.x_labels(value);
        self
    }

    /// Set how many label for the Y axis at most
    /// - `value`: The maximum desired number of labels in the Y axis
    pub fn y_labels(&mut self, value: usize) -> &mut Self {
        self.style.y_labels(value);
        self
    }

    /// Set the formatter function for the X label text
    /// - `fmt`: The formatter function
    pub fn x_label_formatter(&mut self, fmt: &'b dyn Fn(&X::ValueType) -> String) -> &mut Self {
        self.style.x_label_formatter(fmt);
        self
    }

    /// Set the formatter function for the Y label text
    /// - `fmt`: The formatter function
    pub fn y_label_formatter(&mut self, fmt: &'b dyn Fn(&Y::ValueType) -> String) -> &mut Self {
        self.style.y_label_formatter(fmt);
        self
    }

    /// Set the axis description's style. If not given, use label style instead.
    /// - `style`: The text style that would be applied to descriptions
    pub fn axis_desc_style<T: IntoTextStyle<'b>>(&mut self, style: T) -> &mut Self {
        self.style
            .axis_desc_style(style.into_text_style(&self.style.parent_size));
        self
    }

    /// Set the X axis's description
    /// - `desc`: The description of the X axis
    pub fn x_desc<T: Into<String>>(&mut self, desc: T) -> &mut Self {
        self.style.x_desc(desc);
        self
    }

    /// Set the Y axis's description
    /// - `desc`: The description of the Y axis
    pub fn y_desc<T: Into<String>>(&mut self, desc: T) -> &mut Self {
        self.style.y_desc(desc);
        self
    }

    /// Draw the axes for the secondary coordinate system
    pub fn draw(&mut self) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
        self.style.draw()
    }

    /// Set the label style for the secondary axis
    pub fn label_style<T: IntoTextStyle<'b>>(&mut self, style: T) -> &mut Self {
        self.style.label_style(style);
        self
    }
}

/// The struct that is used for tracking the configuration of a mesh of any chart
pub struct MeshStyle<'a, 'b, X: Ranged, Y: Ranged, DB>
where
    DB: DrawingBackend,
{
    pub(super) parent_size: (u32, u32),
    pub(super) draw_x_mesh: bool,
    pub(super) draw_y_mesh: bool,
    pub(super) draw_x_axis: bool,
    pub(super) draw_y_axis: bool,
    pub(super) x_label_offset: i32,
    pub(super) y_label_offset: i32,
    pub(super) n_x_labels: usize,
    pub(super) n_y_labels: usize,
    pub(super) axis_desc_style: Option<TextStyle<'b>>,
    pub(super) x_desc: Option<String>,
    pub(super) y_desc: Option<String>,
    pub(super) line_style_1: Option<ShapeStyle>,
    pub(super) line_style_2: Option<ShapeStyle>,
    pub(super) axis_style: Option<ShapeStyle>,
    pub(super) label_style: Option<TextStyle<'b>>,
    pub(super) format_x: &'b dyn Fn(&X::ValueType) -> String,
    pub(super) format_y: &'b dyn Fn(&Y::ValueType) -> String,
    pub(super) target: Option<&'b mut ChartContext<'a, DB, RangedCoord<X, Y>>>,
    pub(super) _pahtom_data: PhantomData<(X, Y)>,
    pub(super) x_tick_size: [i32; 2],
    pub(super) y_tick_size: [i32; 2],
}

impl<'a, 'b, X, Y, DB> MeshStyle<'a, 'b, X, Y, DB>
where
    X: Ranged,
    Y: Ranged,
    DB: DrawingBackend,
{
    /// Set all the tick mark to the same size
    /// `value`: The new size
    pub fn set_all_tick_mark_size<S: SizeDesc>(&mut self, value: S) -> &mut Self {
        let size = value.in_pixels(&self.parent_size);
        self.x_tick_size = [size, size];
        self.y_tick_size = [size, size];
        self
    }

    pub fn set_tick_mark_size<S: SizeDesc>(
        &mut self,
        pos: LabelAreaPosition,
        value: S,
    ) -> &mut Self {
        *match pos {
            LabelAreaPosition::Top => &mut self.x_tick_size[0],
            LabelAreaPosition::Bottom => &mut self.x_tick_size[1],
            LabelAreaPosition::Left => &mut self.y_tick_size[0],
            LabelAreaPosition::Right => &mut self.y_tick_size[1],
        } = value.in_pixels(&self.parent_size);
        self
    }

    /// The offset of x labels. This is used when we want to place the label in the middle of
    /// the grid. This is useful if we are drawing a histogram
    /// - `value`: The offset in pixel
    pub fn x_label_offset<S: SizeDesc>(&mut self, value: S) -> &mut Self {
        self.x_label_offset = value.in_pixels(&self.parent_size);
        self
    }

    /// The offset of y labels. This is used when we want to place the label in the middle of
    /// the grid. This is useful if we are drawing a histogram
    /// - `value`: The offset in pixel
    pub fn y_label_offset<S: SizeDesc>(&mut self, value: S) -> &mut Self {
        self.y_label_offset = value.in_pixels(&self.parent_size);
        self
    }

    /// Disable the mesh for the x axis.
    pub fn disable_x_mesh(&mut self) -> &mut Self {
        self.draw_x_mesh = false;
        self
    }

    /// Disable the mesh for the y axis
    pub fn disable_y_mesh(&mut self) -> &mut Self {
        self.draw_y_mesh = false;
        self
    }

    /// Disable drawing the X axis
    pub fn disable_x_axis(&mut self) -> &mut Self {
        self.draw_x_axis = false;
        self
    }

    /// Disable drawing the Y axis
    pub fn disable_y_axis(&mut self) -> &mut Self {
        self.draw_y_axis = false;
        self
    }

    /// Set the style definition for the axis
    /// - `style`: The style for the axis
    pub fn axis_style<T: Into<ShapeStyle>>(&mut self, style: T) -> &mut Self {
        self.axis_style = Some(style.into());
        self
    }
    /// Set how many labels for the X axis at most
    /// - `value`: The maximum desired number of labels in the X axis
    pub fn x_labels(&mut self, value: usize) -> &mut Self {
        self.n_x_labels = value;
        self
    }

    /// Set how many label for the Y axis at most
    /// - `value`: The maximum desired number of labels in the Y axis
    pub fn y_labels(&mut self, value: usize) -> &mut Self {
        self.n_y_labels = value;
        self
    }

    /// Set the style for the coarse grind grid
    /// - `style`: This is the fcoarse grind grid style
    pub fn line_style_1<T: Into<ShapeStyle>>(&mut self, style: T) -> &mut Self {
        self.line_style_1 = Some(style.into());
        self
    }

    /// Set the style for the fine grind grid
    /// - `style`: The fine grind grid style
    pub fn line_style_2<T: Into<ShapeStyle>>(&mut self, style: T) -> &mut Self {
        self.line_style_2 = Some(style.into());
        self
    }

    /// Set the style of the label text
    /// - `style`: The text style that would be applied to the labels
    pub fn label_style<T: IntoTextStyle<'b>>(&mut self, style: T) -> &mut Self {
        self.label_style = Some(style.into_text_style(&self.parent_size));
        self
    }

    /// Set the formatter function for the X label text
    /// - `fmt`: The formatter function
    pub fn x_label_formatter(&mut self, fmt: &'b dyn Fn(&X::ValueType) -> String) -> &mut Self {
        self.format_x = fmt;
        self
    }

    /// Set the formatter function for the Y label text
    /// - `fmt`: The formatter function
    pub fn y_label_formatter(&mut self, fmt: &'b dyn Fn(&Y::ValueType) -> String) -> &mut Self {
        self.format_y = fmt;
        self
    }

    /// Set the axis description's style. If not given, use label style instead.
    /// - `style`: The text style that would be applied to descriptions
    pub fn axis_desc_style<T: IntoTextStyle<'b>>(&mut self, style: T) -> &mut Self {
        self.axis_desc_style = Some(style.into_text_style(&self.parent_size));
        self
    }

    /// Set the X axis's description
    /// - `desc`: The description of the X axis
    pub fn x_desc<T: Into<String>>(&mut self, desc: T) -> &mut Self {
        self.x_desc = Some(desc.into());
        self
    }

    /// Set the Y axis's description
    /// - `desc`: The description of the Y axis
    pub fn y_desc<T: Into<String>>(&mut self, desc: T) -> &mut Self {
        self.y_desc = Some(desc.into());
        self
    }

    /// Draw the configured mesh on the target plot
    pub fn draw(&mut self) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
        let mut target = None;
        std::mem::swap(&mut target, &mut self.target);
        let target = target.unwrap();

        let default_mesh_color_1 = RGBColor(0, 0, 0).mix(0.2);
        let default_mesh_color_2 = RGBColor(0, 0, 0).mix(0.1);
        let default_axis_color = RGBColor(0, 0, 0);
        let default_label_font = FontDesc::new(
            "Arial",
            f64::from((12i32).percent().max(12).in_pixels(&self.parent_size)),
        );

        let mesh_style_1 = self
            .line_style_1
            .clone()
            .unwrap_or_else(|| (&default_mesh_color_1).into());
        let mesh_style_2 = self
            .line_style_2
            .clone()
            .unwrap_or_else(|| (&default_mesh_color_2).into());
        let axis_style = self
            .axis_style
            .clone()
            .unwrap_or_else(|| (&default_axis_color).into());

        let label_style = self
            .label_style
            .clone()
            .unwrap_or_else(|| default_label_font.into());

        let axis_desc_style = self
            .axis_desc_style
            .clone()
            .unwrap_or_else(|| label_style.clone());

        target.draw_mesh(
            (self.n_y_labels * 10, self.n_x_labels * 10),
            &mesh_style_2,
            &label_style,
            |_| None,
            self.draw_x_mesh,
            self.draw_y_mesh,
            self.x_label_offset,
            self.y_label_offset,
            false,
            false,
            &axis_style,
            &axis_desc_style,
            self.x_desc.clone(),
            self.y_desc.clone(),
            self.x_tick_size,
            self.y_tick_size,
        )?;

        target.draw_mesh(
            (self.n_y_labels, self.n_x_labels),
            &mesh_style_1,
            &label_style,
            |m| match m {
                MeshLine::XMesh(_, _, v) => Some((self.format_x)(v)),
                MeshLine::YMesh(_, _, v) => Some((self.format_y)(v)),
            },
            self.draw_x_mesh,
            self.draw_y_mesh,
            self.x_label_offset,
            self.y_label_offset,
            self.draw_x_axis,
            self.draw_y_axis,
            &axis_style,
            &axis_desc_style,
            None,
            None,
            self.x_tick_size,
            self.y_tick_size,
        )
    }
}
