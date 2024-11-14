use crate::math::{Mesh, Triangle, Vec4};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};

pub fn load_model(path: String) -> Mesh {
    // println!("Working path: {}", current_dir().unwrap().display());
    // for path_element in read_dir(".").unwrap() {
    //     println!("File/folder: {}", path_element.unwrap().path().display())
    // }
    //
    let mut vertices = vec![];
    let mut triangles = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            let line_tokens: Vec<&str> = line.split_whitespace().collect();
            if line.starts_with("v ") {
                vertices.push(Vec4::new3d(line_tokens[1].parse().unwrap(),
                                          line_tokens[2].parse().unwrap(),
                                          line_tokens[3].parse().unwrap()))
            } else if line.starts_with("f ") {
                let v1_ind: usize = line_tokens[1].split("/").collect::<Vec<&str>>()[0].parse::<usize>().unwrap() - 1;
                let v2_ind: usize = line_tokens[2].split("/").collect::<Vec<&str>>()[0].parse::<usize>().unwrap() - 1;
                let v3_ind: usize = line_tokens[3].split("/").collect::<Vec<&str>>()[0].parse::<usize>().unwrap() - 1;
                triangles.push(Triangle::new(vertices[v1_ind].clone(), vertices[v2_ind].clone(), vertices[v3_ind].clone()));

                if line_tokens.len() > 4 { // this is a quad
                    let v4_ind: usize = line_tokens[4].split("/").collect::<Vec<&str>>()[0].parse::<usize>().unwrap() - 1;
                    triangles.push(Triangle::new(vertices[v1_ind].clone(), vertices[v3_ind].clone(), vertices[v4_ind].clone()));
                }
            }
        }

        return Mesh { triangles };
    }

    return cube();
}

fn cube() -> Mesh {
    Mesh {
        triangles: vec![
            // front
            Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                          Vec4::new3d(-0.5, 0.5, 1.5),
                          Vec4::new3d(0.5, 0.5, 1.5)),
            Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                          Vec4::new3d(0.5, 0.5, 1.5),
                          Vec4::new3d(0.5, -0.5, 1.5)),
            //     // back
            Triangle::new(Vec4::new3d(-0.5, -0.5, 2.5),
                          Vec4::new3d(-0.5, 0.5, 2.5),
                          Vec4::new3d(0.5, 0.5, 2.5)),
            Triangle::new(Vec4::new3d(-0.5, -0.5, 2.5),
                          Vec4::new3d(0.5, 0.5, 2.5),
                          Vec4::new3d(0.5, -0.5, 2.5)),
            // left
            Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                          Vec4::new3d(-0.5, -0.5, 2.5),
                          Vec4::new3d(-0.5, 0.5, 2.5)),
            Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                          Vec4::new3d(-0.5, 0.5, 2.5),
                          Vec4::new3d(-0.5, 0.5, 1.5)),
            // right
            Triangle::new(Vec4::new3d(0.5, -0.5, 1.5),
                          Vec4::new3d(0.5, -0.5, 2.5),
                          Vec4::new3d(0.5, 0.5, 2.5)),
            Triangle::new(Vec4::new3d(0.5, -0.5, 1.5),
                          Vec4::new3d(0.5, 0.5, 2.5),
                          Vec4::new3d(0.5, 0.5, 1.5)),
            // top
            Triangle::new(Vec4::new3d(-0.5, 0.5, 1.5),
                          Vec4::new3d(-0.5, 0.5, 2.5),
                          Vec4::new3d(0.5, 0.5, 2.5)),
            Triangle::new(Vec4::new3d(-0.5, 0.5, 1.5),
                          Vec4::new3d(0.5, 0.5, 2.5),
                          Vec4::new3d(0.5, 0.5, 1.5)),
            // bot
            Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                          Vec4::new3d(-0.5, -0.5, 2.5),
                          Vec4::new3d(0.5, -0.5, 2.5)),
            Triangle::new(Vec4::new3d(-0.5, -0.5, 1.5),
                          Vec4::new3d(0.5, -0.5, 2.5),
                          Vec4::new3d(0.5, -0.5, 1.5)),
        ]
    }
}

fn read_lines(path: String) -> io::Result<Lines<BufReader<File>>>
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
