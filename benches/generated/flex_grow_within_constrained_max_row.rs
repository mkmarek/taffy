pub fn compute() {
    let mut stretch = stretch2::Stretch::new();
    let node00 = stretch
        .new_node(
            stretch2::style::Style {
                flex_shrink: 1f32,
                flex_basis: stretch2::style::Dimension::Points(100f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    height: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                max_size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
}
