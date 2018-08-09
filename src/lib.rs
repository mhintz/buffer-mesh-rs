extern crate cgmath;

use cgmath::{Point2, Point3, Vector3, Vector4};

pub enum Attrib {
    Position,
    Normal,
    Tangent,
    Bitangent,
    Color,
    TexCoord0,
    TexCoord1,
    TexCoord2,
    TexCoord3,
    BoneIndex,
    BoneWeight,
    Custom0,
    Custom1,
    Custom2,
    Custom3,
}

pub struct TriMeshFormat {
    pub has_normals: bool,
    pub has_colors: bool,
    pub has_tex_coords: bool,
}

impl TriMeshFormat {
    pub fn new() -> TriMeshFormat {
        TriMeshFormat {
            has_normals: false,
            has_colors: false,
            has_tex_coords: false,
        }
    }

    pub fn with_normals(mut self) -> TriMeshFormat {
        self.has_normals = true;
        self
    }

    pub fn with_colors(mut self) -> TriMeshFormat {
        self.has_colors = true;
        self
    }

    pub fn with_tex_coords(mut self) -> TriMeshFormat {
        self.has_tex_coords = true;
        self
    }
}

impl Default for TriMeshFormat {
    fn default() -> TriMeshFormat {
        TriMeshFormat::new().with_normals().with_tex_coords()
    }
}

#[derive(Default)]
pub struct TriMesh {
    pub fmt: TriMeshFormat,
    pub indexes: Vec<u32>,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub colors: Vec<f32>,
    pub tex_coords: Vec<f32>,
}

impl TriMesh {
    pub fn new() -> TriMesh {
        TriMesh {
            fmt: Default::default(),
            indexes: Vec::new(),
            positions: Vec::new(),
            normals: Vec::new(),
            colors: Vec::new(),
            tex_coords: Vec::new(),
        }
    }

    pub fn new_with_fmt(fmt: TriMeshFormat) -> TriMesh {
        TriMesh {
            fmt: fmt,
            ..Default::default()
        }
    }

    pub fn add_index(&mut self, idx: u32) {
        self.indexes.push(idx);
    }

    pub fn append_position(&mut self, pt: &Point3<f32>) {
        self.positions.push(pt.x);
        self.positions.push(pt.y);
        self.positions.push(pt.z);
    }

    pub fn append_normal(&mut self, norm: &Vector3<f32>) {
        self.normals.push(norm.x);
        self.normals.push(norm.y);
        self.normals.push(norm.z);
    }

    pub fn append_color(&mut self, color: &Vector4<f32>) {
        self.colors.push(color.x);
        self.colors.push(color.y);
        self.colors.push(color.z);
        self.colors.push(color.w);
    }

    pub fn append_tex_coord(&mut self, coord: &Point2<f32>) {
        self.tex_coords.push(coord.x);
        self.tex_coords.push(coord.y);
    }
}
