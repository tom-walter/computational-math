# Essence of Linear Algebra

## Chapter 1: Vectors
### What's a vector? — 3 Perspectives
1. Physics
    - an arrow sitting and pointing anywhere in space
    - defined by a direction and length
2. Computer Science
    - an ordered lists of numbers (order matters)
3. Mathematcis
    - anything that can be added or multiplied

### Vector Intuition
- an arrow inside a coordinate system with its tail at the origin
    - has same dimensions as the coordinate system
- coordinates of the tip of the arrow are the ordered list of numberrs
- i.e. these numbers tell us how to get from origin (tail) to arrrow's tip
- to distinguish them from points in coordinate system, we use square brackets
    - $\vec{v} =\begin{bmatrix} 1 \\ 2 \end{bmatrix}$

### Addition
$$\begin{bmatrix} x_1 \\ y_1 \end{bmatrix} + \begin{bmatrix} x_2 \\ y_2 \end{bmatrix} = \begin{bmatrix} x_1 + x_2 \\ y_1 + y_2 \end{bmatrix}$$
- physics
    - 4-step movement: follow the 2 elements to the tip of 1st vector and from there follow 2 elements of 2nd vector to the tip of the new vector
- computer science
    - ordered list of number: it's matching up the the terms of each vector and adding them together
$$\begin{bmatrix} 1 \\ 2 \end{bmatrix} + \begin{bmatrix} 3 \\ -1 \end{bmatrix} = \begin{bmatrix} 1 + 3 \\ 2 + (-1) \end{bmatrix} = \begin{bmatrix} 4 \\ 1 \end{bmatrix}$$

### Multiplication
- multiplying a vector by a number is stretching, squishing or flipping the vector along its original
    - stretching for n > 1
    - squishing for 0 < n < 1
    - with flipping for negative n 
- mathetically, this is called **scaling** and the number called **scalar**
$$2 \cdot \begin{bmatrix} x \\ y \end{bmatrix} = \begin{bmatrix} 2x \\ 2y \end{bmatrix}$$
- applied that means mulitplying each component of the vector by the scalar

## Chapter 2: Basis Vector, Linear Combinations and Spans 
### Basis Vectors
- we defined vectors as arrow in coordinate system
- think of each coordinate of the vector as scalar itself
- in an xy-coordinate system, there are two special vectors
    1. $\hat{i}$, the unit vector pointing in x-direction
    2. $\hat{j}$, the unit vector pointing in y-direction
- then the x-coordinate of our vector stretches $\hat{i}$ and the y-coordinate stretches $\hat{j}$
    - $\begin{bmatrix} (x)\hat{i} \\ (y)\hat{j} \end{bmatrix} = (x)\hat{i} + (y)\hat{j}$
- $\hat{i}$ and $\hat{j}$ are the **basis vectors** of the xy-coordinate system
    - by scaling these basis vectors you can reach every possible vector in the coordinate system
    - this is the implicit understanding of coordinate system (technical definition at chapter's end)

### Linear Combination & Span
- $a, b$ are scalars and  vary over all real numbers 
$$a\vec{v} + b\vec{w}$$
- with this understanding, doing both addition and scalar multiplication on two vectors is called a **linear combination**
    - if you fix one scalar in this example and sample the other one freely, the tip of the resulting vectors will draw a line
- the **span** of $\vec{v}$ and $\vec{w}$ is the set all of their linear combinations
    - i.e. covering all coordinate pairs in our xy-coordinate space

### Vectors vs Points
- visualizing all vectors as arrows get becomes crowded, so we do show vectors points; the point being the tip of the arrow

### Spans in 3D Space
1. what's the span of 2 vectors in 3D space (a xyz-coordinate system)?
    - their span is still all the linear combination of those 2 vectors
    - but this span will trace out plane (a flat sheet) through the origin of this 3D space 
2. what's the span of 3 vectors in 3D space?
    - mathematically, it's the linear combination $a\vec{v} + b\vec{w}+ c\vec{u}$
    - if you add a new vector to the previous and vary its linear combinations, then it moves that plane of the original vectors through 3D space
    - thus covering all coordinate triplets in ourr xyz-coordinate space
- what if one vector is redundant meaning, it sits on the same line as another vector?
    - in 2D space, the span becomes a line
    - in 3D space, the span becomes the plane again
    - mathetically, redundant vectors are called **linearly dependent**
    - $\vec{u} = a\vec{v} + b\vec{w}$
    - or it is expressed as a time linear combination of an existing vector and thus already part of the span
- but if each vector adds a dimension to the span, then the vectors are **linearly independent**
    - $\vec{u} \ne a\vec{v} + b\vec{w}$

### Technical Definition of Basis
- > the basis of a vector space is a set of linearly independent vectors that span the full space
