#[test]
fn align_stretch_should_size_based_on_parent() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(20f32),
                height: taffy::style::Dimension::Points(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00 = taffy
        .new_with_children(taffy::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, &[node000])
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                justify_content: Some(taffy::style::JustifyContent::Center),
                flex_grow: 0f32,
                flex_shrink: 1f32,
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(100f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 100f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 20f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node00.data(), 20f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node00.data(), 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node000.data(), 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node000.data(), 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000.data(), 0f32, location.y);
}