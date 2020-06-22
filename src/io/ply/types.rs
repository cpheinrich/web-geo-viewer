/*
Copyright 2020 Martin Buck

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall
be included all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//! Module for interal types for IO operations of the ply file format

use core::convert::TryFrom;

use std::{fmt, io::Error as ioError};

//------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub enum Type {
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Float,
    Double,
}

impl Type {
    #[inline(always)]
    pub fn size_bytes(&self) -> usize {
        match self {
            Self::Char => 1,
            Self::UChar => 1,
            Self::Short => 2,
            Self::UShort => 2,
            Self::Int => 4,
            Self::UInt => 4,
            Self::Float => 4,
            Self::Double => 8,
        }
    }
}

impl TryFrom<&[u8]> for Type {
    type Error = PlyError;

    fn try_from(x: &[u8]) -> PlyResult<Self> {
        match x {
            b"char" => Ok(Self::Char),
            b"uchar" => Ok(Self::UChar),
            b"uint8" => Ok(Self::UChar),
            b"uint32" => Ok(Self::UInt),
            b"int32" => Ok(Self::Int),
            b"uint16" => Ok(Self::UShort),
            b"int16" => Ok(Self::Short),
            b"short" => Ok(Self::Short),
            b"ushort" => Ok(Self::UShort),
            b"int" => Ok(Self::Int),
            b"uint" => Ok(Self::UInt),
            b"float" | b"float32" => Ok(Self::Float),
            b"double" | b"float64" => Ok(Self::Double),
            _ => Err(PlyError::InvalidType(
                std::str::from_utf8(x).unwrap_or("").to_string(),
            )),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum Xyz {
    X,
    Y,
    Z,
}

//------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum VertexOrder {
    Xyz,
    Xzy,
    Yxz,
    Yzx,
    Zxy,
    Zyx,
}

impl TryFrom<[Xyz; 3]> for VertexOrder {
    type Error = PlyError;

    fn try_from(x: [Xyz; 3]) -> PlyResult<Self> {
        match x {
            [Xyz::X, Xyz::Y, Xyz::Z] => Ok(Self::Xyz),
            [Xyz::X, Xyz::Z, Xyz::Y] => Ok(Self::Xzy),
            [Xyz::Y, Xyz::X, Xyz::Z] => Ok(Self::Yxz),
            [Xyz::Y, Xyz::Z, Xyz::X] => Ok(Self::Yzx),
            [Xyz::Z, Xyz::X, Xyz::Y] => Ok(Self::Zxy),
            [Xyz::Z, Xyz::Y, Xyz::X] => Ok(Self::Zyx),
            _ => Err(PlyError::InvalidVertexDimensionDefinition),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Default, Debug)]
pub struct BytesWords {
    pub bytes: usize,
    pub words: usize,
}

//------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum VertexType {
    Float,
    Double,
}

impl TryFrom<Type> for VertexType {
    type Error = PlyError;

    fn try_from(x: Type) -> PlyResult<Self> {
        match x {
            Type::Float => Ok(Self::Float),
            Type::Double => Ok(Self::Double),
            _ => Err(PlyError::InvalidVertexType),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FaceType {
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Float,
    Double,
}

impl TryFrom<Type> for FaceType {
    type Error = PlyError;

    fn try_from(x: Type) -> PlyResult<Self> {
        match x {
            Type::Char => Ok(Self::Char),
            Type::UChar => Ok(Self::UChar),
            Type::Short => Ok(Self::Short),
            Type::UShort => Ok(Self::UShort),
            Type::Int => Ok(Self::Int),
            Type::UInt => Ok(Self::UInt),
            Type::Double => Ok(Self::Double),
            Type::Float => Ok(Self::Float),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct VertexFormat {
    pub order: VertexOrder,
    pub first: VertexType,
    pub snd: VertexType,
    pub third: VertexType,
    pub before: BytesWords,
    pub between_first_snd: BytesWords,
    pub between_snd_third: BytesWords,
    pub after: BytesWords,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct FaceFormat {
    pub name: String,
    pub count: FaceType,
    pub index: FaceType,
}

//------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum Format {
    Ascii,
    LittleEndian,
    BigEndian,
}

//------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub enum HeaderReadState {
    Meta,
    Vertex,
    Face,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Header {
    pub format: Format,
    pub n_vertices: usize,
    pub n_faces: usize,
    pub vertex_format: VertexFormat,
    pub face_format: Vec<FaceFormat>,
    pub texture_name: Option<String>,
}

//------------------------------------------------------------------------------

/// Error type for .ply file operations
pub enum PlyError {
    LoadStartNotFound,
    LoadFormatNotFound,
    LoadWrongPropertyCount,
    LoadVertexIndexDefinitionNotFound,
    LoadHeaderInvalid,
    LoadVertexCountNotFound,
    LoadFaceCountNotFound,
    LoadVertexCountIncorrect,
    AccessFile,
    ColorArrayIncorrectLength,
    //@todo these all could name the affected line
    InvalidType(String),
    InvalidVertexType, //@todo would be better to name the issue
    InvalidFaceType,   //@todo would be better to name the issue
    InvalidMeshIndices(Option<usize>),
    LineParse(usize),
    InvalidProperty(usize),
    InvalidVertex(usize),
    PropertyLineLocation(usize),
    FaceStructure,
    InvalidVertexDimensionDefinition,
}

/// Result type for .ply file operations
pub type PlyResult<T> = std::result::Result<T, PlyError>;

impl fmt::Debug for PlyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LoadStartNotFound => write!(f, "Start of .ply header not found"),
            Self::LoadFormatNotFound => write!(f, "Format of .ply missing or not supported"),
            Self::LoadWrongPropertyCount => {
                write!(f, "Property count of .ply missing or not supported")
            }
            Self::LoadVertexIndexDefinitionNotFound => {
                write!(f, "Index definition in .ply not found")
            }
            Self::LoadHeaderInvalid => write!(f, "Header of .ply seems to be invalid"),
            Self::LoadVertexCountNotFound => write!(f, "Vertex count of .ply not found"),
            Self::LoadFaceCountNotFound => write!(f, "Face count of .ply not found"),
            Self::LoadVertexCountIncorrect => write!(f, "Vertex count of .ply not found"),
            Self::ColorArrayIncorrectLength => {
                write!(f, "The provided color array has an incorrect length")
            }
            Self::InvalidType(x) => write!(f, "Invalid type in header '{}'", x),
            Self::InvalidVertexType => write!(f, "Invalid vertex type in header"),
            Self::InvalidFaceType => write!(f, "Invalid face type in header"),
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::InvalidMeshIndices(opt_x) => match opt_x {
                Some(x) => write!(f, "File contains invalid mesh indices on line {}", x),
                None => write!(f, "File contains invalid mesh indices"),
            },
            Self::InvalidProperty(x) => write!(f, "Invalid property on line {}", x),
            Self::InvalidVertex(x) => write!(f, "Invalid vertex definition on line {}", x),
            Self::InvalidVertexDimensionDefinition => {
                write!(f, "Invalid order / definition of vertex dimension order")
            }
            Self::PropertyLineLocation(x) => write!(
                f,
                "Found property line at unexpected location on line {}",
                x
            ),
            Self::FaceStructure => write!(
                f,
                "Invalid face structure, only supporting 3 vertices per face"
            ),
        }
    }
}

impl From<ioError> for PlyError {
    fn from(_error: ioError) -> Self {
        PlyError::AccessFile
    }
}
