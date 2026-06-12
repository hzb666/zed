use gpui::{Bounds, ContentMask, Corners, Pixels, point, px, size};

fn mask(x: f32, y: f32, width: f32, height: f32, radius: f32) -> ContentMask<Pixels> {
    ContentMask {
        bounds: Bounds {
            origin: point(px(x), px(y)),
            size: size(px(width), px(height)),
        },
        corner_radii: Corners::all(px(radius)),
    }
}

#[test]
fn content_mask_intersection_inherits_only_shared_corners() {
    let parent = mask(0., 0., 100., 100., 7.);
    let rows = mask(0., 38., 100., 62., 0.);

    let intersection = rows.intersect(&parent);

    assert_eq!(px(0.), intersection.corner_radii.top_left);
    assert_eq!(px(0.), intersection.corner_radii.top_right);
    assert_eq!(px(7.), intersection.corner_radii.bottom_right);
    assert_eq!(px(7.), intersection.corner_radii.bottom_left);
}

#[test]
fn content_mask_intersection_does_not_round_detached_edges() {
    let parent = mask(0., 0., 100., 100., 7.);
    let table = mask(0., 38., 70., 62., 0.);

    let intersection = table.intersect(&parent);

    assert_eq!(px(0.), intersection.corner_radii.top_left);
    assert_eq!(px(0.), intersection.corner_radii.top_right);
    assert_eq!(px(0.), intersection.corner_radii.bottom_right);
    assert_eq!(px(7.), intersection.corner_radii.bottom_left);
}

#[test]
fn content_mask_intersection_clamps_radii_to_intersection_size() {
    let parent = mask(0., 0., 100., 100., 40.);
    let thin_child = mask(0., 90., 100., 10., 0.);

    let intersection = thin_child.intersect(&parent);

    assert_eq!(px(5.), intersection.corner_radii.bottom_right);
    assert_eq!(px(5.), intersection.corner_radii.bottom_left);
}

#[test]
fn rounded_content_mask_contains_only_visible_corner_region() {
    let rounded = mask(0., 0., 100., 100., 20.);

    assert!(!rounded.contains(&point(px(1.), px(1.))));
    assert!(rounded.contains(&point(px(20.), px(20.))));
    assert!(rounded.contains(&point(px(50.), px(50.))));
}
