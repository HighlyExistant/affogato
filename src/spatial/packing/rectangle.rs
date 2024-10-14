use std::{cmp::Ordering, ops::Div};

use crate::{geometry::Square, linear::FVec2};

pub fn row_pack_rectangles(rectangles: &Vec<Square<f32>>, cut_off: f32) -> Vec<Square<f32>> {
    let mut height = rectangles.iter().enumerate().map(|(i, v)|{
        let width = v.width();
        let height = v.height();
        
        (width, height)
    }).collect::<Vec<_>>();
    
    height.sort_by(|a, b| if a.1 < b.1 {
        Ordering::Greater
    } else if a.1 > b.1 {
        Ordering::Less
    } else {
        Ordering::Equal
    });
    let mut new_rectangles = vec![Square::default(); rectangles.len()];
    let mut x_pos = 0.0;
    let mut y_pos = 0.0;
    let mut largest_h_this_row = 0.0;
    for (i, (w, h)) in height.into_iter().enumerate() {
        if (x_pos+w) > cut_off {
            y_pos += largest_h_this_row;
            x_pos = 0.0;
            largest_h_this_row = 0.0;
        }
        
        new_rectangles[i] = Square::new(FVec2::new(x_pos, y_pos), FVec2::new(x_pos+w, y_pos+h));
        x_pos += w;
        
        if h > largest_h_this_row {
            largest_h_this_row = h;
        }
    }
    new_rectangles
}