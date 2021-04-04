# Linearisation around a fixed point


# Model

## Constants
Motor lift coefficient ![](https://render.githubusercontent.com/render/math?math=\color{%23666}l), where

![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large%20l*rpm=F_{lift})

Motor torque coefficient ![](https://render.githubusercontent.com/render/math?math=\color{%23666}q), where

![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large%20q*rpm=\tau_{motor})

Motor positions relative to drone center of mass

![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large%20\vec{r}_{m1..m4}=\begin{pmatrix}\vec{r}_{mi,x}%5C%5C\vec{r}_{mi,y}%5C%5C\vec{r}_{mi,z}\end{pmatrix})

Diagonal inertia tensor ![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20I).

## Dynamics

Position as vector

![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large%20\vec{r}=\begin{pmatrix}\vec{r}_x%5C%5C\vec{r}_y%5C%5C\vec{r}_z\end{pmatrix})

Orientation as axis angle

![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large%20\vec{\theta}=\begin{pmatrix}\vec{\theta}_x%5C%5C\vec{\theta}_y%5C%5C\vec{\theta}_z\end{pmatrix})

Angular acceleration

![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large%20\ddot{\vec{\theta}}=)