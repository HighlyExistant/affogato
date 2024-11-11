use std::{cmp::Ordering, ops::Div};

use crate::{geometry::Rect, linear::FVec2};
pub struct RowPackingInfo<'a> {
    pub width: &'a mut f32,
    pub height: &'a mut f32,
    pub x_margin: f32,
    pub y_margin: f32,
}
pub fn row_pack_rectangles(rectangles: &Vec<Rect<f32>>, cut_off: f32, info: Option<RowPackingInfo>) -> Vec<Rect<f32>> {
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
    let mut new_rectangles = vec![Rect::default(); rectangles.len()];
    let mut x_pos = 0.0;
    let mut y_pos = 0.0;
    let mut largest_h_this_row = 0.0;
    let (x_margin, y_margin) = info.as_ref().map(|v|(v.x_margin, v.y_margin)).unwrap_or_default();
    for (i, (w, h)) in height.into_iter().enumerate() {
        if (x_pos+w) > cut_off {
            y_pos += largest_h_this_row+y_margin;
            x_pos = 0.0;
            largest_h_this_row = 0.0;
        }
        
        new_rectangles[i] = Rect::new(FVec2::new(x_pos, y_pos), FVec2::new(x_pos+w, y_pos+h));
        x_pos += w+x_margin;
        
        if h > largest_h_this_row {
            largest_h_this_row = h;
        }
    }
    if let Some(info) = info {
        *info.width = cut_off;
        *info.height = y_pos+largest_h_this_row;
    }
    new_rectangles
}

pub fn row_pack_rectangles_pairs(rectangles: &Vec<Rect<f32>>, cut_off: f32, info: Option<RowPackingInfo>) -> Vec<(Rect<f32>, u32)> {
    let mut height = rectangles.iter().enumerate().map(|(i, v)|{
        let width = v.width();
        let height = v.height();
        
        (width, height, i as u32)
    }).collect::<Vec<_>>();
    
    height.sort_by(|a, b| if a.1 < b.1 {
        Ordering::Greater
    } else if a.1 > b.1 {
        Ordering::Less
    } else {
        Ordering::Equal
    });
    let mut new_rectangles = vec![(Rect::default(), 0); rectangles.len()];
    let (x_margin, y_margin) = info.as_ref().map(|v|(v.x_margin, v.y_margin)).unwrap_or_default();
    let mut x_pos = 0.0;
    let mut y_pos = 0.0;
    let mut largest_h_this_row = 0.0;
    for (i, (w, h, idx)) in height.into_iter().enumerate() {
        if (x_pos+w) > cut_off {
            y_pos += largest_h_this_row+y_margin;
            x_pos = 0.0;
            largest_h_this_row = 0.0;
        }
        
        new_rectangles[i] = (Rect::new(FVec2::new(x_pos, y_pos), FVec2::new(x_pos+w, y_pos+h)), idx);
        x_pos += w+x_margin;
        
        if h > largest_h_this_row {
            largest_h_this_row = h;
        }
    }
    if let Some(info) = info {
        *info.width = cut_off;
        *info.height = y_pos+largest_h_this_row;
    }
    new_rectangles
}