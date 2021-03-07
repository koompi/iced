use crate::{backend, Backend, Primitive, Renderer};
use iced_native::table::{self, TableData, TableColumn};
use iced_native::mouse;
use iced_native::{
    Layout, Point, Rectangle, Color, Background, HorizontalAlignment, VerticalAlignment, Vector
};

pub use iced_native::table::State;
pub use iced_style::table::{Style, StyleSheet};
/// A container that distributes its contents vertically.
pub type Table<'a, Message, Backend> =
    iced_native::Table<'a, Message, Renderer<Backend>>;

impl<B> table::Renderer for Renderer<B>
where B: Backend + backend::Text
{
   type Style = Box<dyn StyleSheet>;
   const DEFAULT_PADDING: u16 = 6;
   const DEFAULT_HEADER_SPACING: u16 = 1;

   fn draw<T: TableData>(
      &mut self,
      _defaults: &Self::Defaults,
      layout: Layout<'_>,
      cursor_position: Point,
      _viewport: &Rectangle,
      columns: &[TableColumn],
      data: &[T],
      is_orderable: bool,
      text_size: u16,
      padding: u16,
      font: Self::Font,
      style: &Self::Style,
   ) -> Self::Output {
      let mut children = layout.children();
      let header_layout = children.next().unwrap();
      let divider_bounds = children.next().unwrap().bounds();
      let body_layout = children.next().unwrap();
      let header_bounds = header_layout.bounds();
      let header_mouse_over = header_bounds.contains(cursor_position);
      let styling = if header_mouse_over {
         style.header_hoverd()
      } else {
         style.active()
      };

      let background = Primitive::Quad {
         bounds: layout.bounds(),
         background: styling.background,
         border_color: styling.border_color,
         border_width: styling.border_width,
         border_radius: styling.border_radius,
      };
      let divider = Primitive::Quad {
         bounds: divider_bounds,
         background: Background::Color(Color::TRANSPARENT),
         border_color: styling.border_color,
         border_width: styling.border_width,
         border_radius: 0.0,
      };
      let header_background = Primitive::Quad {
         bounds: header_bounds,
         background: styling.header_background,
         border_color: Color::TRANSPARENT,
         border_width: 0.0,
         border_radius: styling.border_radius,
      };

      let header_section = Primitive::Group {
         primitives: columns
            .iter()
            .zip(header_layout.children())
            .map(|(column, layout)| {
               let bounds = layout.bounds();
               Primitive::Text {
                  content: column.formatted_sortable_column().to_owned(),
                  size: f32::from(text_size),
                  font,
                  color: styling.text_color,
                  bounds: Rectangle {
                     x: bounds.x + f32::from(padding),
                     y: bounds.center_y(),
                     ..bounds
                  },
                  horizontal_alignment: HorizontalAlignment::Left,
                  vertical_alignment: VerticalAlignment::Center,
               }
            })
            .collect(),
      };

      let mut body_records = Vec::with_capacity(data.len());
      for (record, record_layout) in data.iter().zip(body_layout.children()) {
         // let record_bg = if idx%2==0 {
         //    Primitive::Quad {
         //       bounds: record_layout.bounds(),
         //       background: styling.header_background,
         //       border_color: Color::TRANSPARENT,
         //       border_radius: styling.border_radius,
         //       border_width: 0.0
         //    } 
         // } else {
         //    Primitive::None
         // };
         let record_cells = columns
            .iter()
            .map(|c| c.name.as_str())
            .map(|name| record.get_field_value(name))
            .filter_map(|h| h.ok())
            .zip(record_layout.children())
            .fold(Vec::with_capacity(columns.len()), |mut record_cells, (value, cell_layout)| {
               let bounds = cell_layout.bounds();
               let text = Primitive::Text {
                  content: serde_json::to_string(&value).unwrap(),
                  // content: serde_json::from_value(value).unwrap(),
                  size: f32::from(text_size),
                  font,
                  color: styling.text_color,
                  bounds: Rectangle {
                     x: bounds.x + f32::from(padding),
                     y: bounds.center_y(),
                     ..bounds
                  },
                  horizontal_alignment: HorizontalAlignment::Left,
                  vertical_alignment: VerticalAlignment::Center,
               };
               record_cells.push(
                  Primitive::Clip {
                     bounds,
                     content: Box::new(text),
                     offset: Vector::new(0, 0)
                  }
               );
               record_cells
            });
         let record = Primitive::Group{ primitives: record_cells };
         body_records.push(record);
      }
      let body_section = Primitive::Group{ primitives: body_records };
      (
         Primitive::Group{ primitives: vec![background, header_background, body_section, divider, header_section] },
         if header_mouse_over && is_orderable {
            mouse::Interaction::Pointer
         } else {
            mouse::Interaction::default()
         },
      )
   }
}
