pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(0.55f32),
                    height: taffy::style::Dimension::Percent(0.15f32),
                    ..Size::auto()
                },
                position: taffy::geometry::Rect {
                    right: taffy::style::LengthPercentageAuto::Percent(0.2f32),
                    bottom: taffy::style::LengthPercentageAuto::Percent(0.1f32),
                    ..Rect::auto()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(500f32),
                    height: taffy::style::Dimension::Points(500f32),
                    ..Size::auto()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
