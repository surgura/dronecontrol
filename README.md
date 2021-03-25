# Drone control
Simulation of and control algorithm for quad rotor drone.

## Mathematical model
Rotation of drone in axis-angle representation.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{\theta}=\theta%20\underrightarrow{e},\lvert%20e\rvert=1)

Second derivative of rotation assuming a single motor. Arm ![](https://render.githubusercontent.com/render/math?math=\color{%23666}\underrightarrow{r_F}) originates from drone center of gravity. ![](https://render.githubusercontent.com/render/math?math=I) is the diagonal intertia tensor of the drone.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\ddot{\underrightarrow{\theta}}=I^{-1}(\underrightarrow{r_F}%20\times%20\underrightarrow{F}))

Acceleration of drone from a single motor. Motor force is rotated by ![](https://render.githubusercontent.com/render/math?math=\theta).

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{\alpha}=\frac{1}{m}\overline{\underrightarrow{F}}=\frac{1}{m}(\cos(\theta)\underrightarrow{F}%2B\sin(\theta)(\underrightarrow{e}\times%20\underrightarrow{F})%2B\underrightarrow{e}(1-\cos(\theta))(\underrightarrow{e}\cdot%20\underrightarrow{F})))
 
Position of drone.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\ddot{\underrightarrow{r}}=\underrightarrow{\alpha})

## Time domain solution
Because ![](https://render.githubusercontent.com/render/math?math=\color{%23666}\ddot{\underrightarrow{\theta}}) is contant, integration is straightforward.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{\theta}(t)=\frac{1}{2}t^2\ddot{\underrightarrow{\theta}}%2Bt\dot{\underrightarrow{\theta}}(0)%2B\underrightarrow{\theta}(0))

Derive angle ![](https://render.githubusercontent.com/render/math?math=\color{%23666}\theta).

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\theta(t)=\sqrt{(\frac{1}{2}t^2\ddot{\underrightarrow{\theta}}_x%2Bt\dot{\underrightarrow{\theta}}_x(0)%2B\underrightarrow{\theta}_x(0))^2%2B(\frac{1}{2}t^2\ddot{\underrightarrow{\theta}}_y%2Bt\dot{\underrightarrow{\theta}}_y(0)%2B\underrightarrow{\theta}_y(0))^2%2B(\frac{1}{2}t^2\ddot{\underrightarrow{\theta}}_z%2Bt\dot{\underrightarrow{\theta}}_z(0)%2B\underrightarrow{\theta}_z(0))^2})

Rotational axis ![](https://render.githubusercontent.com/render/math?math=\color{%23666}e).

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{e}(t)=\frac{\underrightarrow{\theta}(t)}{\theta(t)})

Position of drone. ![](https://render.githubusercontent.com/render/math?math=\color{%23666}\alpha) has been split in three parts that can be integrated seperately.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{r}(t)=\frac{1}{m}\displaystyle\int\int%20\underrightarrow{a}(t)%2B\underrightarrow{b}(t)%2B\underrightarrow{c}(t)dtdt)

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{a}(t)=\cos(\theta(t))\underrightarrow{F})

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{b}(t)=\sin(\theta(t))\underrightarrow{e}(t)\times%20\underrightarrow{F})

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{c}(t)=(1-\cos(\theta(t)))(\underrightarrow{e}(t)\cdot%20\underrightarrow{F})\underrightarrow{e}(t))

It is not possible to analytically find a solution to this integral, so estimation is required.

## Integral estimation
The most straightforward way is to simply evaluate ![](https://render.githubusercontent.com/render/math?math=\color{%23666}\alpha) at different intervals, using a variant of the methods described [here](https://tutorial.math.lamar.edu/classes/calcii/approximatingdefintegrals.aspx).

However, instead of evaluating the complete integral it is possible to only evaluate ![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\theta(t)) and ![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{e}(t)) and use the obtained values to replace complex parts of ![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{\alpha}(t)) with simple linear interpolations. This simplifies the integral to the point where it can be solved analytically. For example, linearization of the root ![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\theta(t)):

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\overline{\theta}(q,w,t)=\theta(q)%2B\frac{\theta(w)-\theta(q)}{w-q}(t-q))

Where ![](https://render.githubusercontent.com/render/math?math=\color{%23666}q) and ![](https://render.githubusercontent.com/render/math?math=\color{%23666}w) are the points to estimate ![](https://render.githubusercontent.com/render/math?math=\color{%23666}\theta(t)) between.  Assuming the function parts will be evaluated ![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20n) n times over the required timespan, let

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\theta_n(t)=k_nt%2Bl_n)

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{e_n}(t)=\underrightarrow{o_n}t%2B\underrightarrow{p_n})

This simplifies the drone position integral to

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{a_n}(t)=\cos(k_nt%2Bl_n)\underrightarrow{F})

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{b_n}(t)=\sin(k_nt%2Bl_n)(\underrightarrow{o_n}t%2B\underrightarrow{p_n})\times%20\underrightarrow{F})

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{c_n}(t)=(1-\cos(k_nt%2Bl_n))((\underrightarrow{o_n}t%2B\underrightarrow{p_n})\cdot%20\underrightarrow{F})(\underrightarrow{o_n}t%2B\underrightarrow{p_n}))

## Resulting formulas
TODO