extern crate image;
use image::{GenericImageView, ImageBuffer, Rgba};
use std::time::Instant;

pub mod dijkstra;

pub mod structs;

use structs::Node;

fn graph_from_img(path: &str) -> Vec<Node>{
    // Create variables
    let img = image::open(path).expect("File not found!");
    let mut img_vec: Vec<Vec<bool>> = Vec::new();
    let mut nodes: Vec<Node> = vec![Node{
        x: 0, 
        y: 1, 
        lengths: Vec::new(), 
        neighbours: Vec::new()
    }];
    let mut img_to_node: Vec<Vec<usize>> = Vec::new();

    // Create boolean array from image
    for (_x, y, pixel) in img.pixels(){
        if y >= img_vec.len() as u32{
            img_vec.push(Vec::new());
            img_to_node.push(Vec::new());
        }
        img_vec[y as usize].push(pixel.0[0] != 0);
        img_to_node[y as usize].push(usize::MAX);
    }

    // Create nodes from boolean array
    let mut top: Vec<(usize, i32)> = Vec::new();
    for _ in 0..img_vec.len()-1{
        top.push((0, 0));
    }

    let mut latest = 0;
    let mut h_len = 0;

    for y in 1..img_vec.len()-1{
        latest = (y != 1) as usize * usize::MAX;
        h_len = (y == 1) as i32;
        for x in 1..img_vec[0].len()-1{
            if img_vec[y][x]{
                if img_vec[y][x-1] != img_vec[y][x+1] || img_vec[y-1][x] != img_vec[y+1][x]{
                    nodes.push(Node{
                        x: x as i32,
                        y: y as i32,
                        lengths: Vec::new(),
                        neighbours: Vec::new()
                    });

                    // Join to last node horizontally
                    let current = nodes.len() - 1;
                    if latest != usize::MAX{
                        nodes[latest].lengths.push(h_len);
                        nodes[latest].neighbours.push(current);

                        nodes[current].lengths.push(h_len);
                        nodes[current].neighbours.push(latest);
                    }
                    latest = current;
                    h_len = 0;

                    // Join to last node vertically
                    if top[x].0 != 0{
                        nodes[top[x].0].lengths.push(top[x].1);
                        nodes[top[x].0].neighbours.push(current);

                        nodes[current].lengths.push(top[x].1);
                        nodes[current].neighbours.push(top[x].0);
 
                    }
                    top[x].0 = current;
                    top[x].1 = 0;
                }
                h_len += 1;
                top[x].1 += 1;
            }else{
                latest = usize::MAX;
                top[x].0 = 0;
            }
        }
    }

    // Connect final node
    nodes.push(Node{
        x: (img_vec.len() - 1) as i32,
        y: (img_vec[0].len() - 2) as i32,
        lengths: Vec::new(),
        neighbours: Vec::new()
    });

    let current = nodes.len() - 1;

    nodes[latest].lengths.push(h_len);
    nodes[latest].neighbours.push(current);

    nodes[current].lengths.push(h_len);
    nodes[current].neighbours.push(latest); 

    return nodes;
}

fn main() {
    // Declare variables
    let img_path = "src/maze-0.png";
    let nodes: Vec<Node> = graph_from_img(img_path);

    let start_node: usize = 0;
    let end_node: usize = nodes.len() - 1;

    let start = Instant::now();
    let path = dijkstra::solve(&nodes, start_node, end_node);

    println!("Total length: {}", path.1);
    println!("Time elapsed: {:?}", start.elapsed());
    
    // Display laberint solution
    let img = image::open(img_path).expect("File not found!");
    let (w, h) = img.dimensions();
    let mut output = ImageBuffer::new(w, h);

    for (x, y, pixel) in img.pixels(){
        output.put_pixel(x, y, pixel);
    }

    let length = path.1;
    let mut current_length = 0;
    for i in (1..path.0.len()).rev(){
        let index = path.0[i];
        let next_index = path.0[i-1];

        let x_distance = nodes[next_index].x - nodes[index].x;
        let y_distance = nodes[next_index].y - nodes[index].y;

        let dir = (x_distance.abs() > 0) as i32;
        let distance = dir * x_distance + (dir - 1).abs() * y_distance;

        let mut iterator = 0..distance;

        if distance < 0{
            iterator = distance..1;
        }

        for j in iterator{
            let lerp_value = current_length as f32 / length as f32 * 255.0;
            let pixel_color = Rgba([lerp_value as u8, 0, 255 - lerp_value as u8, 255]);
            let x = nodes[index].x + j * dir;
            let y = nodes[index].y + j * (dir - 1).abs();

            output.put_pixel(x as u32, y as u32, pixel_color);

            current_length += 1;
        }
    }

    output.put_pixel(nodes[end_node].x as u32, nodes[end_node].y as u32, Rgba([255, 0, 0, 255]));

    output.save("src/output.png").unwrap();
}
