# FrostyEngine
A simple 2d game engine in rust.

Can be allowed to handle events with it's entity component system and physics engine, or
a person can create their own event loop with window.get_event_loop().

### Design Principles


### The ECS

A game is broken down into scenes which hold entities. These entities have components which determine how they function. These components can function as a way to store information about an entity (such as the sprite that it should be rendered as), as flags that determine whether certain scene rules should apply to it (such as whether it has collision or not), or as a way of accepting user input (such as with a character controller)

Entities can be created free of a scene, but for an entity to function it needs to be added to a scene. 

### Using Shaders

Rendering is handled through the rust library WGPU. A shader struct needs to be made for any new shader that is loaded. The struct is able to take certain vertices, as described to it, then render them to the screen or to a texture. 