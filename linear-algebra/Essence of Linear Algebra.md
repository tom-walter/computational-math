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

## Chapter 3: Linear Transformations and Matrices
### Matrices as Linear Transformations
- a transformation is a function like $f(x)$ which takes an input and gives an output
- in lin alg, such a function $L(\vec{v})$ takes an input vector and gives an output vector
- visually speaking, a transformation is linear if
    - all lines remrain lines
    - the origins remains in place
    - i.e. keeps grid lines parallel and evenly spaced
- numerically, we ca think of this transformation of the grid as the transformation of vector space with $\hat{i}, \hat{j}$ 
    - thus, we only need to understand how the basis vectors are transformed to deduce how any other vector will transformed
- example
    - transformation on $\hat{i}, \hat{j}$
    $$\hat{i} → \begin{bmatrix} 1 \\ -2 \end{bmatrix} \text{ } \hat{j} → \begin{bmatrix} 3 \\ 0 \end{bmatrix}$$
    - applying  $\hat{i}$ to x-coordinate and  $\hat{j}$ to y-coordinate
    $$\begin{bmatrix} x \\ y \end{bmatrix} → x \begin{bmatrix} 1 \\ -2 \end{bmatrix} + y \begin{bmatrix} 3 \\ 0 \end{bmatrix} = \begin{bmatrix} 1x + 3y \\ -2x + 0y \end{bmatrix}$$
- this means, we only need 4 numbers (for a 2D space) to describe any linear transformation
- these 4 numbers are expressed as **2x2 matrix**, where the two columns describe how $\hat{i}, \hat{j}$ are transformed, e.g.
    $$\begin{bmatrix} 3 & 2 \\ -2 & 1 \end{bmatrix}$$
    - now we can apply columns of the matrix to the corresponding elements of a vector
    $$\vec{v} = \begin{bmatrix} 5 \\ 7 \end{bmatrix}$$
    $$5 \begin{bmatrix} 3 \\ -2 \end{bmatrix} + 7 \begin{bmatrix} 2 \\ 1 \end{bmatrix} $$
    $$\begin{bmatrix} 15 + 14 \\ -10 + 7 \end{bmatrix} = \begin{bmatrix} 29 \\ -3 \end{bmatrix}$$

### 2x2 Matrix
- let's generalize this matrix as linear transformation of vectors
- the column $a, c$ tranforms the $\hat{i}$ and the column $b, d$ the $\hat{j}$ basis vectors
    $$\begin{bmatrix} a & b \\ c & d \end{bmatrix} \text{ }  \begin{bmatrix} x \\ y \end{bmatrix}$$
    $$x \begin{bmatrix} a \\ c \end{bmatrix} + y \begin{bmatrix} c \\ d \end{bmatrix} = \begin{bmatrix} ax + by \\ cx + dy \end{bmatrix}$$
- these steps descibe **matrix-vector multiplication**

### Special Cases
- counter-clockwise rotation of 90°
    $$\begin{bmatrix} 0 & -1 \\ 1 & 0 \end{bmatrix}$$
- "shear": $\hat{i}$ remains fixed but $\hat{j}$ moves over
    $$\begin{bmatrix} 1 & 1 \\ 0 & 1 \end{bmatrix}$$
- linearly dependent: transforms 2D vector space into 1D vector space
    $$\begin{bmatrix} 2 & -2 \\ 1 & -1 \end{bmatrix}$$

### Summary
- linear transformation move around space such that gridlines remain parellel and evenly spaced, and the origin remains fixed
- matrices give us tool to describe these transformation in general and matrix multiplication with vector to apply them

### Technical Definition of Linear
- > a transformation is linear if it satisfies 2 properties
    > 1. additivity: $L(\vec{v} + \vec{w}) = L(\vec{v}) + L(\vec{w})$ 
    > 2. scaling: $L(c\vec{v}) = cL(\vec{v})$

## Chapter 4: Matrix Multiplication as Composition
### Recap
- linear transformation are functions with input vector and output vector $L(\vec{v}) = \vec{w}$
- we can think about the as stretching space such that grid lines stay parallel & evenly spaced, and so that origin remains fixed
- linear transformation are determined by where they take the base vectors $\hat{i}, \hat{j}$
- convention is a matrix where each column determines where base vector lands after transformation and the transformation is done by matrix-vector multiplication
    $$\begin{bmatrix} a & b \\ c & d \end{bmatrix} \begin{bmatrix} x \\ y \end{bmatrix} = x \begin{bmatrix} a \\ c \end{bmatrix} + y \begin{bmatrix} c \\ d \end{bmatrix} = \begin{bmatrix} ax + by \\ cx + dy \end{bmatrix}$$

### Composition
- how can we describe multitple (consecutive) linear transformations?
- i.e. 1st rotate, 2nd shear → gives a new distinct linear transformation
- this is called a **composition** of a rotation and a shear
- can be expressed as its own matrix with final position of $\hat{i}, \hat{j}$
    - you can always think of matrix multiplication visually as where the base vectors will finally land (even after multiple transformations)
- individual steps should be equal to composition method
    - 1st apply rotation matrix to vector
    - 2nd apply shear matrix to vector
    - or only apply composition matrix to vector 
    $$\begin{bmatrix} 1 & 1 \\ 0 & 1 \end{bmatrix} \left( \begin{bmatrix} 0 & -1 \\ 1 & 0 \end{bmatrix} \begin{bmatrix} x \\ y \end{bmatrix} \right) = \begin{bmatrix} 1 & -1 \\ 1 & 0 \end{bmatrix} \begin{bmatrix} x \\ y \end{bmatrix} $$
- the composition matrix is the product of the original matrices
    $$\begin{bmatrix} 1 & 1 \\ 0 & 1 \end{bmatrix} \begin{bmatrix} 0 & -1 \\ 1 & 0 \end{bmatrix}  = \begin{bmatrix} 1 & -1 \\ 1 & 0 \end{bmatrix} $$
    - multipying two matrices has geometric meaning of transforming the base vectors by one then another
    - order of operations is right-to-left because of function notation $f(g(x))$

### Example
- multiply $M_2=\begin{bmatrix} 0 & 2 \\ 1 & 0 \end{bmatrix}$ by $M_1 = \begin{bmatrix} 1 & -2 \\ 1 & 0 \end{bmatrix}$
    $$\begin{bmatrix} 0 & 2 \\ 1 & 0 \end{bmatrix} \begin{bmatrix} 1 & -2 \\ 1 & 0 \end{bmatrix}  = \begin{bmatrix} ? & ? \\ ? & ? \end{bmatrix} $$
- step 1:
    - where does $\hat{i}$ land?
    - multiply $M_2$ by **left column** of $M_1$  using the matrix vector calculation
    $$\begin{bmatrix} 0 & 2 \\ 1 & 0 \end{bmatrix}  \begin{bmatrix} 1 \\ 1 \end{bmatrix} = 1 \begin{bmatrix} 0 \\ 1 \end{bmatrix} + 1 \begin{bmatrix} 2 \\ 0 \end{bmatrix} =  \begin{bmatrix} 2 \\ 1 \end{bmatrix} $$
    - this will be the **left column** of the composition matrix
- step 2:
    - where does $\hat{j}$ land?
    - multiply $M_2$ by **right column** of $M_1$  using the matrix vector calculation
    $$\begin{bmatrix} 0 & 2 \\ 1 & 0 \end{bmatrix}  \begin{bmatrix} -2 \\ 0 \end{bmatrix} = -2 \begin{bmatrix} 0 \\ 1 \end{bmatrix} + 0 \begin{bmatrix} 2 \\ 0 \end{bmatrix} =  \begin{bmatrix} 0 \\ -2 \end{bmatrix} $$
    - this will be the **right column** of the composition matrix
- finally, the composition matrix is
    $$\begin{bmatrix} 0 & 2 \\ 1 & 0 \end{bmatrix} \begin{bmatrix} 1 & -2 \\ 1 & 0 \end{bmatrix}  = \begin{bmatrix} 2 & 0 \\ 1 & -2 \end{bmatrix} $$

### Generalization
- matrix-matrix multiplication
    $$\begin{bmatrix} a & b \\ c & d \end{bmatrix} \begin{bmatrix} e & f \\ g & h \end{bmatrix}  = \begin{bmatrix} ? & ? \\ ? & ? \end{bmatrix} $$
- 1: left column, where $\hat{i}$ lands
    $$\begin{bmatrix} a & b \\ c & d \end{bmatrix} \begin{bmatrix} e \\ g \end{bmatrix} = e \begin{bmatrix} a \\ c \end{bmatrix} + g \begin{bmatrix} b \\ h \end{bmatrix} =  \begin{bmatrix} ae + bg \\ ce + dg \end{bmatrix} $$
- 2: right column, where $\hat{j}$ lands
    $$\begin{bmatrix} a & b \\ c & d \end{bmatrix} \begin{bmatrix} f \\ h \end{bmatrix} = f \begin{bmatrix} a \\ c \end{bmatrix} + h \begin{bmatrix} b \\ h \end{bmatrix} =  \begin{bmatrix} af + bh \\ cf + dh \end{bmatrix} $$
- finally: composition matrix
        $$\begin{bmatrix} a & b \\ c & d \end{bmatrix} \begin{bmatrix} e & f \\ g & h \end{bmatrix}  = \begin{bmatrix} ae + bg & af + bh \\ ce + dg & cf + dh \end{bmatrix} $$
- order of operations matters $M_2 M_1 \ne M_1 M_2$, always from left to right
- but matrix multiplication is associative $(AB)C = A(BC)$
    - i.e. it doesn't matter if you multiply $AB$ or $BC$ first
    - since the order is fixed from left to right, it doesn't matter where you put the parenthesis
- the visual proof is better and easier to follow/understand than the symbolic proof 
    - visualize matrix multiplication as where the base vectors will land

## Chapter 5: 3D Linear Transformation
### Linear Transformation to Three Dimensions
- the concepts from two dimensions carry over to high dimensions 
- 3D linear transformation
    - is a function that takes a 3D input vector and returns a 3D output vector
    - keeps the grid lines parallel & evenly spaced, the origins remains fixed
    - can be completely described by where the base vectors go
- the 3rd unit vector is $\hat{k}$ (k-hat)

### Transformation as a Matrix
- we can visually think about the transformation in terms of where each basis vector goes
    $$\hat{i} → \begin{bmatrix} 1 \\ 0 \\ -1 \end{bmatrix} \hat{j} → \begin{bmatrix} 1 \\ 1 \\ 0 \end{bmatrix} \hat{k} → \begin{bmatrix} 1 \\ 0 \\ 1 \end{bmatrix} $$
    - i.e. $\hat{i}$ moves one unit on the x-axis, no unit on the y-axis, and one flipped unit on the z-axis
- collect these movements in a **3x3 matrix** where each column correspond to a basis vector
    $$\begin{bmatrix} 1 & 1 & 1 \\ 0 & 1 & 0 \\ -1 & 0 & 1 \end{bmatrix}$$
- example: rotation of 90° around the y-axis
    $$\begin{bmatrix} 0 & 0 & 1 \\ 0 & 1 & 0 \\ -1 & 0 & 0 \end{bmatrix}$$

### Application
- **matrix-vector mulitplication**:
    - apply each column from the matrix to the corresponding dimension of the vector
    - then add together the intermediary results
    $$\begin{bmatrix} 0 & 1 & 2 \\ 3 & 4 & 5 \\ 6 & 7 & 8 \end{bmatrix} \begin{bmatrix} x \\ y \\ z \end{bmatrix} = x\begin{bmatrix} 0 \\ 3 \\ 6 \end{bmatrix} + y\begin{bmatrix} 1 \\ 4 \\ 7 \end{bmatrix} +  z\begin{bmatrix} 2 \\ 5 \\ 8 \end{bmatrix}$$

### Notes
- high-dimensional matrix multiplication is important in
    - computer graphics
    - robotics
    - AI/machine learning
