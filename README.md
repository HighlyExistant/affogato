# What is Affogato
Well it's 🍦 + ☕, which are all the things im going to need after writing all of this. Other than that though, it's a personal math project I use inspired by a variety of different math libraries.
# History of the Project
The library is mainly filled with a bunch of things I needed at the time. As I made more projects, more math code was added. The majority of the things I code are graphics related, and even though there are various amazing libraries for linear algebra in rust crates, I felt the need to learn due to my philosophy of needing to understand how every bit of code works. Even though it's meant to be for math, I started adding features to help me with graphics related projects.
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
* Physics calculations. As I work more with physics, i'll require more math functions related to physics, and will most likely add them here, perhaps as an optional feature.
* Graphics quality of life would make my life much easier, so it's currently my priority. Even though it's not necessarily math related, it works very closely with my math code, so I should add it as a feature.
# Inspirations
* [glm](https://www.opengl.org/sdk/libs/GLM/) (OpenGL Mathematics) possibly one of my biggest inspirations, Lots of the Matrix and Vector code is inspired from this library.
* [msdfgen](https://github.com/Chlumsky/msdfgen/tree/master) A smaller but equally important inspiration, from which I derived my Segment code.
* [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html#addingasphere) The Ray class is inspired from this site
* [The Book of Shaders](https://thebookofshaders.com/glossary/?search=reflect) snippets of code for smoothing functions, and other glsl functions were acquired from here
* [This video on FABRIK](https://www.youtube.com/watch?v=UNoX65PRehA&t=685s&ab_channel=EgoMoose) Where I inspired my kinematics physics code