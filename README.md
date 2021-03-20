# Drone control
Simulation of and control algorithm for quad rotor drone.

## Mathematical model
Rotation of drone in axis-angle representation.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{\theta}=\theta%20\underrightarrow{e},\lvert%20e\rvert=1)

Second derivative of rotation assuming a single motor. Arm ![](https://render.githubusercontent.com/render/math?math=\color{%23666}\underrightarrow{r_F}) originates from drone center of gravity. ![](https://render.githubusercontent.com/render/math?math=I) is the diagonal intertia tensor of the drone.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\ddot{\underrightarrow{\theta}}=I^{-1}(\underrightarrow{r_F}%20\times%20\underrightarrow{F}))

Acceleration of drone from a single motor. Motor force is rotated by ![](https://render.githubusercontent.com/render/math?math=\theta).

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{\alpha}=\frac{1}{m}\underrightarrow{F^'}=\frac{1}{m}(\cos(\theta)\underrightarrow{F}%2B\sin(\theta)(\underrightarrow{e}\times%20\underrightarrow{F})%2B\underrightarrow{e}(1-\cos(\theta))(\underrightarrow{e}\cdot%20\underrightarrow{F})))
 
Position of drone.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\ddot{\underrightarrow{r}}=\underrightarrow{\alpha})

## Time domain solution
Because ![](https://render.githubusercontent.com/render/math?math=\color{%23666}\ddot{\underrightarrow{\theta}}) is contant, integration is straightforward.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{\theta}(t)=\frac{1}{2}t^2\ddot{\underrightarrow{\theta}}%2Bt\dot{\underrightarrow{\theta}}(0)%2B\underrightarrow{\theta}(0))

Derive angle ![](https://render.githubusercontent.com/render/math?math=\color{%23666}\theta).

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\theta(t)=\sqrt{(\frac{1}{2}t^2\ddot{\underrightarrow{\theta}}_x%2Bt\dot{\underrightarrow{\theta}}_x(0)%2B\underrightarrow{\theta}_x(0))^2%2B(\frac{1}{2}t^2\ddot{\underrightarrow{\theta}}_y%2Bt\dot{\underrightarrow{\theta}}_y(0)%2B\underrightarrow{\theta}_y(0))^2%2B(\frac{1}{2}t^2\ddot{\underrightarrow{\theta}}_z%2Bt\dot{\underrightarrow{\theta}}_z(0)%2B\underrightarrow{\theta}_z(0))^2})

Rotational axis ![](https://render.githubusercontent.com/render/math?math=\color{%23666}e).

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{e}(t)=\frac{\underrightarrow{\theta}(t)}{\theta(t)})

Velocity of drone. ![](https://render.githubusercontent.com/render/math?math=\color{%23666}\alpha) has been split in the three operands of addition, because their integrals can be found seperately.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{v}=\frac{1}{m}\displaystyle\int%20\underrightarrow{a}(t)%2B\underrightarrow{b}(t)%2B\underrightarrow{c}(t)dt)

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{a}(t)=\cos(\theta(t))\underrightarrow{F})

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{b}(t)=\sin(\theta(t))(\underrightarrow{e}(t)\times%20\underrightarrow{F}))

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{c}(t)=\underrightarrow{e}(t)(1-\cos(\theta(t)))(\underrightarrow{e}(t)\cdot%20\underrightarrow{F}))

### a(t)
![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\displaystyle\int\underrightarrow{a}(t)dt=\displaystyle\int\cos(\theta(t))\underrightarrow{F}dt)

Its not possible to compute this integral, so try approximation by taylor expansion.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\theta(t)=\sqrt{x(t)},%20\ddddot{x}(t)=0)

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20a(t)=\cos(\sqrt{x(t)}))

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\dot{a}(t)=-\frac{\dot{x}(t)}{2\sqrt{x(t)}}\sin(\sqrt{x(t)}))

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\ddot{a}(t)=-(\frac{\dot{x}(t)}{2\sqrt{x(t)}})^2\cos(\sqrt{x(t)})-(\frac{\ddot{x}(t)}{2\sqrt{x(t)}}-\frac{\dot{x}(t)^2}{4x(t)^{\frac{3}{2}}})\sin(\sqrt{x(t)}))

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\dddot{a}(t)=(\frac{3\dot{x}(t)^3}{8x(t)^2}-\frac{3\dot{x}(t)\ddot{x}(t)}{4x(t)})\cos(\sqrt{x(t)})%2B((\frac{\dot{x}(t)}{2\sqrt{x(t)}})^3-\frac{\dddot{x}(t)}{2\sqrt{x(t)}}%2B\frac{3\dot{x}(t)\ddot{x}(t)}{4x(t)^{\frac{3}{2}}}-\frac{3\dot{x}(t)^3}{8x(t)^{\frac{5}{2}}})\sin(\sqrt{x(t)}))

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\ddddot{a}(t)=-(\frac{3\dot{x}(t)^3}{8x(t)^2}-\frac{3\dot{x}(t)\ddot{x}(t)}{4x(t)})(\frac{\dot{x}(t)}{2\sqrt{x(t)}})\sin(\sqrt{x(t)})%2B(\frac{9\ddot{x}(t)\dot{x}(t)^2}{8x(t)^2}-\frac{3\dot{x}(t)^4}{4x(t)^3}-\frac{3\ddot{x}(t)^2%2B3\dot{x}(t)\dddot{x}(t)}{4x(t)}%2B\frac{3\dot{x}(t)^2\ddot{x}(t)}{4x(t)^2})\cos(\sqrt{x(t)}))