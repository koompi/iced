use crate::{
    layout::{Limits, Node}, Element, Hasher, Layout, Length, Point, Rectangle, Size, Widget,
};

#[derive(Debug, Clone, Copy)]
pub enum Overflow {
    Visible,
    Clip,
}

impl Default for Overflow {
    fn default() -> Self {
        Self::Visible
    }
}

impl Overflow {
    pub const ALL: [Overflow; 2] = [Overflow::Visible, Overflow::Clip];
}

/// A widget used to stack multiple widgets overlay.
#[allow(missing_debug_implementations)]
pub struct Stack<'a, Message, Renderer> {
    overflow: Overflow,
    children: Vec<(Element<'a, Message, Renderer>, Option<Point>)>,
}

impl<'a, Message, Renderer> Stack<'a, Message, Renderer> {
    pub fn new() -> Self {
        Self::with_children(Vec::new())
    }

    pub fn with_children(children: Vec<(Element<'a, Message, Renderer>, Option<Point>)>) -> Self {
        Self {
            overflow: Overflow::default(),
            children,
        }
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    pub fn push<E>(mut self, element: E, point: Option<Point>) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.children.push((element.into(), point));
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Stack<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        if self.children.is_empty() {
            Node::new(Size::ZERO)
        } else {
            let mut nodes = Vec::with_capacity(self.children.len());
            let mut height: f32 = 0.;
            let mut width: f32 = 0.;

            for (element, point) in self.children.iter() {
                let mut node = element.layout(renderer, &limits);
                let size = node.size();
                if let Some(point) = point {
                node.move_to(*point);
                match self.overflow {
                    Overflow::Visible => {
                        width = width.max(size.width + point.x);
                        height = height.max(size.height + point.y);
                    }
                    Overflow::Clip => {
                        height = height.max(size.height);
                        width = width.max(size.width);
                    }
                }
                } else {
                height = height.max(size.height);
                width = width.max(size.width);
                }
                nodes.push(node);
            }

            Node::with_children(Size::new(width, height), nodes)
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Renderer::Output {
        renderer.draw(defaults, layout, cursor_position, viewport, &self.overflow, &self.children)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.children.iter().for_each(|(element, _)| {
            element.hash_layout(state);
        })
    }
}

pub trait Renderer: crate::Renderer + Sized {
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        overflow: &Overflow,
        children: &[(Element<'_, Message, Self>, Option<Point>)],
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<Stack<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: Clone + 'a,
{
    fn from(stack: Stack<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(stack)
    }
}