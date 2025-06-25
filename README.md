# What is Affogato
Well it's 🍦 + ☕, which are all the things im going to need after writing all of this. Other than that though, it's a personal math project I use inspired by a variety of different math libraries. I'm most likely going to change this repositories name to something still pertaining to coffee, but maybe not just affogato.
# History of the Project
The library is mainly filled with a bunch of things I needed at the time. As I made more projects, more math code was added. The majority of the things I code are graphics related, and even though there are various amazing libraries for linear algebra in rust crates, I felt the need to learn due to my philosophy of needing to understand how every bit of code works. Even though it's meant to be for math, I started adding features to help me with graphics related projects. After a while I also decided to add features like physics and collisions in their own features, and started adding compatibility with other rust crates, in case I ever need to use them in the future.
# Features
* alloc
This feature is automatically activated, but can be deactivated using `default-features = false`. By deactivating this feature you are removing any code that uses allocators, such as Display trait implementations.
* serde
This is a compatibility feature so that you can use many of the types I've implemented in [serde](https://crates.io/crates/serde).
* rand
This is a compatibility feature that adds random functions to various types like vectors.
* physics
Adds inverse kinematics, rigidbodies and collisions
* godot
This is a compatibility feature for the [godot](https://crates.io/crates/godot) crate which implements From and Into for various types.
* glsl
Although I don't reccomend using it, it adds padding for types like Vector3 and FMat3 so that they are more reliable to use inside of shaders in glsl.
## Linear Algebra
The largest portion of this library is filled with code for vectors and matrices, in fact it's one of the first things I ever needed, and has been left relatively unchanged.
* Vectors
  * Coordinate representation
  * Polar Coordinates
* Matrices
## Geometry
* Segments
* Rays
* Hyperecubes
  * Rectangles
  * Cubes (Cuboid)
* Spheres
* Triangles
* Segments
* Planes
## Spatial Curves
* Z-curve (Morton codes)
## Smoothing Functions
* Smooth interpolation
* Linear interpolation
* Bilinear interpolation
## Algebra
* Complex numbers
* Quaternions
* Formulas
  * Quadratic Formula
  * Cubic Formula
## Inverse Kinematics
* KinematicSegmentList
# Things I might add
* I've been interested in geometric algebra, so I might look into [ganja.js](https://github.com/enkimute/ganja.js?files=1) and add rotors. This might lead to an enire rework of Complex numbers and Quaternions, as well as the addition of hyperbolic numbers (Split complex numbers) and duel numbers, although I currently have no reason to, as it's not something im working on for a project.
* Reworking KinematicSegmentList to use polar coordinates internally.
# Inspirations
* [glm](https://www.opengl.org/sdk/libs/GLM/) (OpenGL Mathematics) possibly one of my biggest inspirations, Lots of the Matrix and Vector code is inspired from this library.
* [msdfgen](https://github.com/Chlumsky/msdfgen/tree/master) A smaller but equally important inspiration, from which I derived my Segment code.
* [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html#addingasphere) The Ray class is inspired from this site
* [The Book of Shaders](https://thebookofshaders.com/glossary/?search=reflect) snippets of code for smoothing functions, and other glsl functions were acquired from here
* [This video on FABRIK](https://www.youtube.com/watch?v=UNoX65PRehA&t=685s&ab_channel=EgoMoose) Where I inspired my kinematics physics code
* [GJK: Collision detection algorithm in 2D/3D](https://winter.dev/articles/gjk-algorithm) Where I got the GJK collision algorithm code from
* [iquilezles articles](https://iquilezles.org/articles/smin/) Where I got some of my sdf and smooth minimum functions.
* [Meshmagick User's Guide](https://lheea.github.io/meshmagick/_modules/meshmagick/inertia.html) Where I got some of my inertia computation code.