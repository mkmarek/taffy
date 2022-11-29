pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(150f32),
                    height: taffy::style::Dimension::Points(80f32),
                    ..Size::auto()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(80f32),
                    height: taffy::style::Dimension::Points(80f32),
                    ..Size::auto()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style { flex_wrap: taffy::style::FlexWrap::Wrap, ..Default::default() },
            &[node00, node01],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_items: taffy::style::AlignItems::FlexStart,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
                    height: taffy::style::Dimension::Points(200f32),
                    ..Size::auto()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
