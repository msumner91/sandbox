# sandbox
This is a sandbox for experimenting with Rust & graphics programming with OpenGL. 

Note this will only work on Windows currently and there is no plan to port this to Vulkan since performance isn't the primary driver.

Currently supports:
1. Rendering terrain from a height-map
2. Loading obj files & render entities/meshes/textures
3. A basic first person camera with movement (W/A/S/D and left click to pan & scroll wheel for zoom)
4. Calculating and rendering of bounding boxes for entities (with sub-meshes in an entity also having their own bounding box)
5. Perform AABB ray intersections (i.e. we can interact with 3D objects with the mouse)
6. Diffuse/specular lighting

Controls:
- W/A/S/D to move camera position, arrow keys for object movement
- Left click and drag to pan camera
- Scroll wheel to zoom in/out
- Right click to cast an invisible ray that can push objects around

Sample images:
![Sandbox 1](/resources/img/sandbox.png)
![Sandbox 2](/resources/img/sandbox2.png)

Prerequisites:
- Since this uses raw GLFW-rs bindings, you must compile GLFW v3.x and place it into a lib folder in the root directory for cargo run/build to work properly. see https://github.com/PistonDevelopers/glfw-rs#using-glfw-rs for more details

Many elements of the object/mesh loading and camera code are heavily based on the wonderful port of learning OpenGL with rust series found here: https://github.com/bwasty/learn-opengl-rs.
