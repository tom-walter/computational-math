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
```math
\begin{bmatrix} x_1 \\ y_1 \end{bmatrix} + \begin{bmatrix} x_2 \\ y_2 \end{bmatrix} = \begin{bmatrix} x_1 + x_2 \\ y_1 + y_2 \end{bmatrix}
```
- physics
    - 4-step movement: follow the 2 elements to the tip of 1st vector and from there follow 2 elements of 2nd vector to the tip of the new vector
- computer science
    - ordered list of number: it's matching up the the terms of each vector and adding them together
```math
\begin{bmatrix} 1 \\ 2 \end{bmatrix} + \begin{bmatrix} 3 \\ -1 \end{bmatrix} = \begin{bmatrix} 1 + 3 \\ 2 + (-1) \end{bmatrix} = \begin{bmatrix} 4 \\ 1 \end{bmatrix}
```

### Multiplication
- multiplying a vector by a number is stretching, squishing or flipping the vector along its original
    - stretching for n > 1
    - squishing for 0 < n < 1
    - with flipping for negative n 
- mathetically, this is called **scaling** and the number called **scalar**
```math
2 \cdot \begin{bmatrix} x \\ y \end{bmatrix} = \begin{bmatrix} 2x \\ 2y \end{bmatrix}
```
- applied that means mulitplying each component of the vector by the scalar

## Chapter 2: Basis Vector, Linear Combinations and Spans 
### Basis Vectors
- we defined vectors as arrow in coordinate system
- think of each coordinate of the vector as scalar itself
- in an xy-coordinate system, there are two special vectors
    1. $\hat{i}$, the unit vector pointing in x-direction
    2. $\hat{j}$, the unit vector pointing in y-direction
- then the x-coordinate of our vector stretches $\hat{i}$ and the y-coordinate stretches $\hat{j}$
```math
\begin{bmatrix} (x)\hat{i} \\ (y)\hat{j} \end{bmatrix} = (x)\hat{i} + (y)\hat{j}
```
- $\hat{i}$ and $\hat{j}$ are the **basis vectors** of the xy-coordinate system
    - by scaling these basis vectors you can reach every possible vector in the coordinate system
    - this is the implicit understanding of coordinate system (technical definition at chapter's end)

### Linear Combination & Span
- $a, b$ are scalars and  vary over all real numbers 
```math
a\vec{v} + b\vec{w}
```
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
```math
\hat{i} → \begin{bmatrix} 1 \\ -2 \end{bmatrix} \text{ } \hat{j} → \begin{bmatrix} 3 \\ 0 \end{bmatrix}
```
    - applying  $\hat{i}$ to x-coordinate and  $\hat{j}$ to y-coordinate
```math
\begin{bmatrix} x \\ y \end{bmatrix} → x \begin{bmatrix} 1 \\ -2 \end{bmatrix} + y \begin{bmatrix} 3 \\ 0 \end{bmatrix} = \begin{bmatrix} 1x + 3y \\ -2x + 0y \end{bmatrix}
```
- this means, we only need 4 numbers (for a 2D space) to describe any linear transformation
- these 4 numbers are expressed as **2x2 matrix**, where the two columns describe how $\hat{i}, \hat{j}$ are transformed, e.g.
```math
M = \begin{bmatrix} 3 & 2 \\ -2 & 1 \end{bmatrix}
```
    - now we can apply columns of the matrix to the corresponding elements of a vector $\vec{v}$
```math
\begin{align*}
M \vec{v} &= \begin{bmatrix} 3 & 2 \\ -2 & 1 \end{bmatrix} \begin{bmatrix} 5 \\ 7 \end{bmatrix} \\
& = 5 \begin{bmatrix} 3 \\ -2 \end{bmatrix} + 7 \begin{bmatrix} 2 \\ 1 \end{bmatrix} \\
& = \begin{bmatrix} 15 + 14 \\ -10 + 7 \end{bmatrix} \\
& = \begin{bmatrix} 29 \\ -3 \end{bmatrix}
\end{align*}
```

### 2x2 Matrix
- let's generalize this matrix as linear transformation of vectors
- the column $a, c$ tranforms the $\hat{i}$ and the column $b, d$ the $\hat{j}$ basis vectors
```math
\begin{bmatrix} a & b \\ c & d \end{bmatrix} \text{ }  \begin{bmatrix} x \\ y \end{bmatrix}\\
x \begin{bmatrix} a \\ c \end{bmatrix} + y \begin{bmatrix} c \\ d \end{bmatrix} = \begin{bmatrix} ax + by \\ cx + dy \end{bmatrix}
```
- these steps descibe **matrix-vector multiplication**

### Special Cases
- counter-clockwise rotation of 90°
```math
\begin{bmatrix} 0 & -1 \\ 1 & 0 \end{bmatrix}
```
- "shear": $\hat{i}$ remains fixed but $\hat{j}$ moves over
```math
\begin{bmatrix} 1 & 1 \\ 0 & 1 \end{bmatrix}
```
- linearly dependent: transforms 2D vector space into 1D vector space
```math
\begin{bmatrix} 2 & -2 \\ 1 & -1 \end{bmatrix}
```

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
```math
\begin{bmatrix} a & b \\ c & d \end{bmatrix} \begin{bmatrix} x \\ y \end{bmatrix} = x \begin{bmatrix} a \\ c \end{bmatrix} + y \begin{bmatrix} c \\ d \end{bmatrix} = \begin{bmatrix} ax + by \\ cx + dy \end{bmatrix}
```

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
```math
\begin{bmatrix} 1 & 1 \\ 0 & 1 \end{bmatrix} \left( \begin{bmatrix} 0 & -1 \\ 1 & 0 \end{bmatrix} \begin{bmatrix} x \\ y \end{bmatrix} \right) = \begin{bmatrix} 1 & -1 \\ 1 & 0 \end{bmatrix} \begin{bmatrix} x \\ y \end{bmatrix} 
```
- the composition matrix is the product of the original matrices
```math
\begin{bmatrix} 1 & 1 \\ 0 & 1 \end{bmatrix} \begin{bmatrix} 0 & -1 \\ 1 & 0 \end{bmatrix}  = \begin{bmatrix} 1 & -1 \\ 1 & 0 \end{bmatrix}
```
- multipying two matrices has geometric meaning of transforming the base vectors by one then another
- order of operations is right-to-left because of function notation $f(g(x))$

### Example
- multiply $M_2=\begin{bmatrix} 0 & 2 \\ 1 & 0 \end{bmatrix}$ by $M_1 = \begin{bmatrix} 1 & -2 \\ 1 & 0 \end{bmatrix}$
```math
\begin{bmatrix} 0 & 2 \\ 1 & 0 \end{bmatrix} \begin{bmatrix} 1 & -2 \\ 1 & 0 \end{bmatrix}  = \begin{bmatrix} ? & ? \\ ? & ? \end{bmatrix}
```
- step 1:
    - where does $\hat{i}$ land?
    - multiply $M_2$ by **left column** of $M_1$  using the matrix vector calculation
```math
\begin{bmatrix} 0 & 2 \\ 1 & 0 \end{bmatrix}  \begin{bmatrix} 1 \\ 1 \end{bmatrix} = 1 \begin{bmatrix} 0 \\ 1 \end{bmatrix} + 1 \begin{bmatrix} 2 \\ 0 \end{bmatrix} =  \begin{bmatrix} 2 \\ 1 \end{bmatrix}
```
    - this will be the **left column** of the composition matrix
- step 2:
    - where does $\hat{j}$ land?
    - multiply $M_2$ by **right column** of $M_1$  using the matrix vector calculation
```math
\begin{bmatrix} 0 & 2 \\ 1 & 0 \end{bmatrix}  \begin{bmatrix} -2 \\ 0 \end{bmatrix} = -2 \begin{bmatrix} 0 \\ 1 \end{bmatrix} + 0 \begin{bmatrix} 2 \\ 0 \end{bmatrix} =  \begin{bmatrix} 0 \\ -2 \end{bmatrix}
```
    - this will be the **right column** of the composition matrix
- finally, the composition matrix is
```math
\begin{bmatrix} 0 & 2 \\ 1 & 0 \end{bmatrix} \begin{bmatrix} 1 & -2 \\ 1 & 0 \end{bmatrix}  = \begin{bmatrix} 2 & 0 \\ 1 & -2 \end{bmatrix}
```

### Generalization
- matrix-matrix multiplication
```math
\begin{bmatrix} a & b \\ c & d \end{bmatrix} \begin{bmatrix} e & f \\ g & h \end{bmatrix}  = \begin{bmatrix} ? & ? \\ ? & ? \end{bmatrix}
```
- 1: left column, where $\hat{i}$ lands
```math
\begin{bmatrix} a & b \\ c & d \end{bmatrix} \begin{bmatrix} e \\ g \end{bmatrix} = e \begin{bmatrix} a \\ c \end{bmatrix} + g \begin{bmatrix} b \\ h \end{bmatrix} =  \begin{bmatrix} ae + bg \\ ce + dg \end{bmatrix}
```
- 2: right column, where $\hat{j}$ lands
```math
\begin{bmatrix} a & b \\ c & d \end{bmatrix} \begin{bmatrix} f \\ h \end{bmatrix} = f \begin{bmatrix} a \\ c \end{bmatrix} + h \begin{bmatrix} b \\ h \end{bmatrix} =  \begin{bmatrix} af + bh \\ cf + dh \end{bmatrix}
```
- finally: composition matrix
```math
\begin{bmatrix} a & b \\ c & d \end{bmatrix} \begin{bmatrix} e & f \\ g & h \end{bmatrix}  = \begin{bmatrix} ae + bg & af + bh \\ ce + dg & cf + dh \end{bmatrix}
```
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

## Chapter 6: The Determinant
### Determinant as Scaling
- linear transformations flip, stretch or squish the vector space
- how much is the space of a given region squished or streched?
- mathematically speaking: how much are **areas scaled** (2D) or the **volumes scaled** (3D)?

### Scaling the Unit Square
- the 1x1 unit square is the square described by the basis vectors $\hat{i}, \hat{j}$ 
- the linear transformation affects the entire space (grid) in the same way, so every square on this grid is scaled in the same way as the unit square
    - follows from fact that grid lines remain parallel and evenly spaced
    - any shape that's not a square can be apprroximated by smaller squares
- the scaling factor of the linear transforrmation called the **determinant**

### Intuition in 2D
- the determinant of a lin. transformation would be 3.0, if it increases the area of the unit square by factor 3
    $$\det \left( \begin{bmatrix} 0.0 & 2.0 \\ -1.5 & 1.0 \end{bmatrix} \right) = 3.0$$
- the determinant would by 0.5 if it squished all areas down by factor 0.5
    $$\det \left( \begin{bmatrix} 0.5 & 0.5 \\ -0.5 & 0.5 \end{bmatrix} \right) = 0.5$$
- the deterrminant of a 2D transformation is 0, if it squishes all space onto a line or single point (because transformation is *linearly dependent*)
- determinants can also be negative
    - when a transformation inverts the orientation changes
    - i.e. if the 2D space was a paper and we flipped it over
    - original $\hat{j}$ is left of $\hat{i}$, if after transformation it is right of $\hat{i}$, the orientation is inverted
    $$\det \left( \begin{bmatrix} 2 & 1 \\ -1 & -3 \end{bmatrix} \right) = -5$$
    - the absolute value still tells you the scaling

### Intuition in 3D
- the determinant tells you how volume gets scaled
- the 1x1x1 unit cube whose edges sit on the basis vectors $\hat{i}, \hat{j}, \hat{k}$ 
- after transformation the warped cube has a changed volume and the shape is called **parallelipiped**
- the unit cube starts with volume of 1 and the determinant is the factor by which all volume are scaled, *the determinant is the volumne of the parallelipiped*
- the deterrminant of a 3D transformation is 0, if it squishes all space into plane, line or single point
- get orientation via right-hand rule
    - thumb points up ↑ for $\hat{k}$
    - index finger points foward ↖ for $\hat{i}$
    - middle finger points left ← for $\hat{j}$

### Visualize and Formalize
<p align="center">
<img src="essence-lin-alg-ch06-parallelogram.png" alt="Parallelogram" width=400/>
</p>

- our unit square gets transformed into some kind of parallelogram
- this can described as
    $$\det \left( \begin{bmatrix} a & b \\ c & d \end{bmatrix} \right) = (a+b) (c+d) -ac -db -2bc $$
    $$\det \left( \begin{bmatrix} a & b \\ c & d \end{bmatrix} \right) = ad - bc$$
- our unit cube gets transformed into a parallelipiped
- the formula is
    $$
    \begin{align*}
    \det \left(\begin{bmatrix}
    a & b & c \\
    d & e & f \\
    g & h & i
    \end{bmatrix}\right)
    =
    a\det
    \left(\begin{bmatrix}
    e & f \\
    h & i
    \end{bmatrix}\right)
    \\
    -
    b\det
    \left(\begin{bmatrix}
    d & f \\
    g & i
    \end{bmatrix}\right)
    \\
    +
    c\det
    \left(\begin{bmatrix}
    d & e \\
    g & h
    \end{bmatrix}\right)
    \end{align*}
    $$
- there are algorithms for computing the determinant of large matrices
    - LU decomposition
    - Laplace expansion

### Quiz
- determinant of a matrix product should be the same as product of the determinants of the original matrices
    $$\det(M_1 M_2) = \det(M_1) \det(M_2)$$
- because the composition matrix transforms vector space in the same way as the original matrices applied consecutively, hence the scaling should be same

## Chapter 7: Linear Systems of Equations
### Learning Goals
- visually understanding inverse matrices, column space, rank, null space
- not the methods / algorithms to compute them such as Gaussian elimination, row echolon form (these resources already exists in adundance)

### Linear Systems of Equations
- linear algebra is important to any technical discipline
    - it lets us solve linear systems of equations
- linear systems of equations have
    - a list of known variables (we want to know)
    - a list of equations relating them
- linearity constraints are that
    - the variables are scaled by a constant
    - the (scaled) variables are added 
    - no interaction of variables, no fancy functions (e.g. no $\sin(x), e^x, xy$)
- example of 3D or 3 unknowns
    - we can organize such a system by lining up the variables and put lingering constants on the right
    - we may have to write out zero or one coefficients to make this "alignment" more explicit
    $$
    \begin{align*}
    2x + 5y + 3z & = -3 \\
    4x + 0y + 8z & = 0 \\
    1x + 3y + 0z & = 2
    \end{align*}
    $$
- this can be written as a single vector equation with
    - a matrix containing all constant coefficients
    - a vector with the unknown variable 
    - another vector with constants 
    $$
    \begin{bmatrix}
    2 & 5 & 3 \\
    4 & 0 & 8 \\
    1 & 3 & 0 \\
    \end{bmatrix}
    \begin{bmatrix} x \\ y \\ z \end{bmatrix} 
    = 
    \begin{bmatrix} -3 \\ 0 \\ 2 \end{bmatrix} 
    $$
    - we denote this $A \bold{\vec{x}} = \bold{\vec{v}}$
- now, we can recognize this as a linear transformation
    - $A$ transforms $\bold{\vec{x}}$ that it will land on $\bold{\vec{v}}$
    $$
    \begin{align*}
    2x + 2y &= -3 \\
    1x + 3y & = -1
    \end{align*}
    $$
    $$
    \begin{bmatrix}
    2 & 2 \\
    1 & 3
    \end{bmatrix}
    \begin{bmatrix} x \\ y \end{bmatrix} 
    = 
    \begin{bmatrix} -4 \\ -1 \end{bmatrix} 
    $$
- to find the unknown variables, we want find the vector $\bold{\vec{x}}$  

### Inverse and Identity Transformation
- we already know $A$ can transform the vector space maintaining its dimension or squish it onto a lower dimension
- mathematically, we have 2 cases
    1. $\det(A) \ne 0$ : A has a non-zero determinant, maintaining dimensions
    2. $\det(A) = 0$ : A has a zero determinant, reducing dimensions
- for case 1
    - there can only be one vector $\bold{\vec{x}}$ that will land on $\bold{\vec{v}}$ after transformation
    - so to find this unknown $\bold{\vec{x}}$, we can reverse the transformation on the known vector $\bold{\vec{v}}$
    - doing a transformation in reverse is its own linear transformation commonly called **inverse of a matrix** denotated as $A^{-1}$
- example roration matrix
    - if $A$ is counter-clockwise roration of 90°, the $A^{-1}$ is a clockwise roration of 90°
    $$
    \begin{align*}
    A = \begin{bmatrix} 0 & -1 \\ 1 & 0  \end{bmatrix} \\
    A^{-1} = \begin{bmatrix} 0 & 1 \\ -1 & 0  \end{bmatrix}
    \end{align*}
    $$
- a matrix mulitplied by its inverse gives you the **identity transformation**
    $$A^{-1} A = \begin{bmatrix} 1 & 0 \\ 0 & 1  \end{bmatrix} $$
    - identity transformation is a linear transforrmation that does noething, i.e.
    - it leaves $\hat{i}, \hat{j}$ were they arer / un-moved
- after finding the inverse, we use it to solve for $\bold{\vec{v}}$
    $$
    \begin{align*}
    A \bold{\vec{x}} &= \bold{\vec{v}} \\
    A^{-1} A \bold{\vec{x}} &= A^{-1} \bold{\vec{v}} \\
    \bold{\vec{x}} &= A^{-1} \bold{\vec{v}} 
    \end{align*}
    $$
    - playing the transformation in reverse on the known, output vector given us the unknown input vector
    - also works the same way in higher dimensions
- considering the 2 cases again
    - as long as $\det(A) \ne 0 → A^{-1}$ exists
    - if $\det(A) = 0$, the space squished down in dimensions and there is no inverse
    - because you cannot unsquish a line to turn it into a plane
    - there is an exception if the solution does live on this lower dimension

### Rank and Column Space
- since matrices can have higher dimensions, the zero determinant cases can take on more than one form of rerducing dimensions, e.g. from 3D to plane (2D), from 3D to line (1D), etc.
    - how can we describe this better?
- if the output of a transformation lands on
    1. a line (1D), we call the transformation rank 1
    2. a plane (2D), we call it rank 2
- **rank** means the number of dimension in the output of a transformation
- for 2x2 matrices, rank 2 is the best it can be → called a **full rank**
- for 3x3 matrices,
    - rank 3 means the dimension have been maintained in 3D
    - rank 2 means the dimension have collapsed from 3D to 2D
- **column space** is the set of all possible outputs of $A \bold{\vec{v}}$ 
    - as the  columns of the matrix tell you where the basis vectors land
    - the span of columns is the "column space"
- thus, rank is more precisely, the number of dimensions in the column space
    - if it's as a high as can be, the matrix is full rank

### Null Space
- the  zero vector $\begin{bmatrix} 0 \\ 0 \end{bmatrix}$ is always in the column space as linear transformations keep the origin fixed
- dimensional reduction
    - for full rank transforrrmations, only zero vector lands on origin
    - but for matrices below full rank a bunch of vectors can lend on zero after transformation
    - for 2x2 matrix reducing to a line, there is a line of vectors "in opposite direction" that are all squished onto zero
    - for 3x3 matrix reudcing to a line, there is a whole plane of vectors that are suqished onto the origin
- the set of vectors landing on the origin is called **null space** or kernel
- for linear systems of equations if $\bold{\vec{v}}$ is the zero vector, the null space gives you all possible solutions to the equation

### Summary
- we looked at linear systems of equations geometrically
- each system has some kind of linear transformation associated
- if this transformation has an inverse, we can use the inverse to solve the system
- the idea of determinants and column space lets us know when a solution even exists
- the idea of null space lets us know what the set of all possible solution can look like
