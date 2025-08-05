pub fn chaikin_algo(points:Vec<(f32,f32)>)->Vec<(f32,f32)>{
    let mut new_points: Vec<(f32, f32)> = Vec::new();
     let (start_x, start_y) = points[0];
    let (end_x,end_y) = points[points.len()-1];

    new_points.push((start_x,start_y));
    if points.len()>3{
        for i in 0..points.len() - 1 {
            let (x1, y1) = points[i];
            let (x2, y2) = points[i + 1];
            let qx=0.75*x1+0.25*x2;
            let qy=0.75*y1+0.25*y2;
            let rx=0.25*x1+0.75*x2;
            let ry=0.25*y1+0.75*y2;
            new_points.push((qx,qy));
            new_points.push((rx,ry));
        }
    }    
    new_points.push((end_x,end_y));
    new_points
}
