#[test]
fn grid_absolute_column_end() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, Taffy};
    let mut taffy: Taffy<crate::TextMeasure> = Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            grid_column: taffy::geometry::Line { start: taffy::style::GridPlacement::Auto, end: line(1i16) },
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(4f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(1f32),
                bottom: taffy::style::LengthPercentageAuto::Length(2f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![length(40f32), length(40f32), length(40f32)],
                grid_template_columns: vec![length(40f32), length(40f32), length(40f32)],
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(40f32),
                    right: taffy::style::LengthPercentage::Length(20f32),
                    top: taffy::style::LengthPercentage::Length(10f32),
                    bottom: taffy::style::LengthPercentage::Length(30f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 180f32, "width of node {:?}. Expected {}. Actual {}", node, 180f32, size.width);
    assert_eq!(size.height, 160f32, "height of node {:?}. Expected {}. Actual {}", node, 160f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 33f32, "width of node {:?}. Expected {}. Actual {}", node0, 33f32, size.width);
    assert_eq!(size.height, 157f32, "height of node {:?}. Expected {}. Actual {}", node0, 157f32, size.height);
    assert_eq!(location.x, 4f32, "x of node {:?}. Expected {}. Actual {}", node0, 4f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node0, 1f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_height()
    );
}
