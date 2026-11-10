# Beyond the vertical slice: candidate milestones

Status: **proposal**. Nothing here is accepted scope.
Milestones 0 through 6 are complete (`STATUS.md`), which is the evidence `docs/roadmap.md` asks for before later features are considered.
Each item below must still be promoted into `docs/roadmap.md` with contracts and exit evidence before work starts, and several of them reopen non-goals listed in `docs/learning-charter.md` (adaptive, symplectic and stiff solvers, automatic differentiation).

All implementation of the reserved areas in `AGENTS.md` stays with the learner.
This document only records what to build, why, and how it would be tested.

## The thesis

The first slice proved that a units-aware source language can reach a calibrated trajectory.
The next level is one idea applied repeatedly:

> **The compiler knows things about the model that a numerical library never can, and the runtime should act on them.**

Dimensions, symmetries, conserved quantities, Jacobian structure and identifiability are all compile-time facts.
A feature belongs here if it turns one of those facts into a solver decision, a rejected program, or a report the user could not have got from SciPy.

## Part A: the five core candidates

### A1. Dimension-typed gradients through the solver

- **What:** Replace the central finite-difference sensitivity in `calibration.rs` with real differentiation through RK4. Every Jacobian entry gets a physical dimension: `d position / d damping` is `m / (kg/s)`, and the compiler derives it.
- **Why it is new:** Dimensional analysis of gradients is rare in simulation tools. A unit mismatch in a loss term becomes a compile error.
- **Test:** The AD gradient matches central differences to a tolerance derived from the step size, and a deliberately wrong unit in a loss term is rejected.
- **Question:** Do you differentiate the discrete steps or the continuous ODE? What does each give up?
- **Extensions worth planning for:** forward sensitivity equations versus a reverse (adjoint) pass, and checkpointing so reverse mode does not store the whole trajectory.

### A2. Automatic nondimensionalization and an identifiability report

- **What:** From the dimension matrix the compiler finds the dimensionless groups (Buckingham pi), rewrites the model in them and calibrates there. For the oscillator, `mass`, `damping` and `stiffness` collapse to a damping ratio and a natural frequency. `stormer fit` reports which combinations the data can recover, with confidence intervals from the Gauss-Newton covariance.
- **Test:** Fitting 2 versus 3 free parameters on synthetic position data reports a rank-deficient Fisher matrix exactly when the groups say it should.
- **Question:** Why does the null space of the dimension matrix tell you which parameter combinations the data cannot separate?
- **Two things to settle first:**
  1. `Dimension` currently stores signed integer exponents. The damping ratio is `c / (2 sqrt(m k))`, so pi groups need rational exponents and the language needs `sqrt` or a power operator.
  2. The precise statement is about the *observation*, not the model alone. Position data carries only length and time, so no combination of parameters with a net mass exponent can be recovered from it. The report should be driven by the dimensions of what was measured. Measuring force as well would make mass identifiable, and the report should change accordingly.

### A3. Invariants as language features, and Størmer's own problem

- **What:** Allow `invariant energy = ...`. The compiler differentiates it symbolically along the flow and proves `d/dt = 0` for conservative models. The runtime then picks the integrator (Störmer-Verlet or another symplectic scheme when the structure allows, RK4 otherwise) and reports drift.
- **Flagship demo:** A charged particle in a magnetic dipole field, the trapped motion behind the aurora, with a Boris-style pusher.
- **Test:** Over long runs RK4 drifts in energy while the symplectic scheme stays bounded.
- **Question:** What does "symplectic" preserve, and why is that not the same as energy conservation?

### A4. Hybrid systems: events with differentiable timing

- **What:** `when position < 0 { velocity = -e * velocity }`. The solver locates the crossing inside a step (dense output plus root-finding) and restarts from the new state.
- **Why it is hard:** Gradients with respect to `e` or the drop height must account for the event time moving (the saltation matrix), and Zeno behaviour must be detected.
- **Test:** Bounce times match the closed form, and the restitution coefficient is recovered from noisy data.

### A5. Stretch: constraints and DAEs

- **What:** A pendulum in Cartesian coordinates with `x^2 + y^2 = L^2` as an equation, and index reduction by the Pantelides algorithm.
- **Why last:** It forces a full symbolic pass over the expression arena.

## Part B: additional candidates

These came out of reading the current code and asking where A1 through A5 stop short.

### B1. Error-controlled adaptive stepping with dense output

- **What:** An embedded pair (Dormand-Prince 5(4)) with a step-size controller and a continuous interpolant.
- **Why:** It is a hard prerequisite for A4, which needs dense output to find event times, and it removes `--dt` as something the user has to guess.
- **Test:** Tolerance proportionality. Tightening the tolerance by 10x reduces the measured global error by roughly the predicted factor on the oscillator's closed form.
- **Question:** When differentiating through an adaptive solver, do you differentiate the step-size controller too? What does the gradient mean if you do not?

### B2. `stormer verify`: self-checking convergence

- **What:** A command that reruns a simulation at `dt`, `dt/2`, `dt/4`, estimates the observed order by Richardson extrapolation and prints an error estimate next to the result.
- **Why:** The charter says a single plausible plot is not evidence. This makes that standard a product feature instead of a test-suite habit. Small, and useful for every later milestone.
- **Test:** Reports order 1 for Euler and 4 for RK4 on the oscillator, and flags a run whose step is in the unstable region.

### B3. Stiffness: implicit solvers fed by the compiler's Jacobian

- **What:** Once A1 exists, the compiler has the exact Jacobian and its sparsity pattern for free. Use them for an implicit method (implicit midpoint or a Rosenbrock scheme) and a stiffness warning.
- **Why:** It is the clearest case of the thesis. Numerical libraries estimate the Jacobian by finite differences; Størmer can derive it with units.
- **Test:** Van der Pol with large `mu` and the Robertson kinetics problem. RK4 needs a step orders of magnitude smaller than the implicit method for the same error.

### B4. From identifiability to experiment design

- **What:** Three steps past A2.
  1. Print the eigenvalue spectrum of the Fisher matrix, so "sloppy" directions are visible, not only exactly singular ones.
  2. Profile likelihood per parameter, which stays honest where the quadratic Fisher approximation does not.
  3. `stormer design`: given a model and a measurement budget, propose *when* and *what* to measure so that the determinant of the Fisher matrix is largest (D-optimal design).
- **Why:** A2 tells the user what they cannot learn. This tells them what experiment would fix it. That closes the loop, and very few tools do it.
- **Test:** For the oscillator, sampling times clustered near zero crossings identify frequency better than damping and the design step moves samples accordingly. Adding a force observation makes mass identifiable.

### B5. Structural identifiability by symbolic observability

- **What:** A2 detects scaling symmetries only. The general test differentiates the observation along the flow (Lie derivatives) and checks the rank of the resulting observability matrix symbolically.
- **Why:** It separates "the model can never tell these apart" from "this data set was too noisy", and it reuses the symbolic differentiation built for A3.
- **Test:** A two-compartment model with a known non-identifiable pair is flagged before any data is loaded.

### B6. Bayesian calibration that reuses the symplectic integrator

- **What:** Hamiltonian Monte Carlo over the parameters, using the A1 gradient.
- **Why it fits unusually well:** HMC's leapfrog step *is* the Störmer-Verlet integrator. The scheme added in A3 to simulate conservative models gets reused to sample the posterior of those same models.
- **Test:** On a linear-Gaussian case the posterior matches the closed form. On the oscillator the credible intervals agree with A2 where the problem is well-posed and widen correctly where it is not.

### B7. Dimension-pruned model discovery

- **What:** Given data and declared units, enumerate candidate right-hand-side terms, discard every term the dimension checker rejects, and fit the survivors with sparse regression (the SINDy idea).
- **Why:** The type system shrinks the hypothesis space before any fitting happens. This is the most research-flavoured item here and the one most likely to be new.
- **Test:** From noisy oscillator data and a library of 30 or so monomials, only the dimensionally valid terms survive pruning and the true two-term law is recovered.

### B8. Unknown terms with typed inputs and outputs

- **What:** `unknown friction(velocity): N`, a small learned function inside an otherwise mechanistic model, with inputs nondimensionalized by the A2 groups and the output dimension enforced.
- **Why:** It is the "universal differential equation" idea with a unit-safe boundary. Needs A1 and A2.
- **Test:** Data generated with quadratic drag, model declared with linear drag plus an unknown term. The learned term matches the missing physics.

### B9. Language growth that the above needs

- Rational dimension exponents and `sqrt` / `pow` (blocks A2).
- Elementary functions with dimension rules: `sin`, `exp` and `log` accept only dimensionless arguments (blocks the dipole demo and most real models).
- Vector states, so the dipole problem is not written as six scalar equations.
- `observe` declarations that state what was measured and in which unit, so A2 and B4 have something to reason about.
- Explicit time dependence and forcing inputs.
- Later: components with typed ports (Modelica-style), which lead naturally into A5.

### B10. Execution: bytecode, native code, and a differential oracle

- **What:** Lower the IR to a register bytecode with common-subexpression elimination, then optionally to native code through Cranelift.
- **Why:** ADR 0002 says interpret before codegen. The interpreter now exists and matches the hard-coded oscillator bit-for-bit, so it can serve as the oracle that every faster backend must agree with.
- **Test:** Bit-identical trajectories between interpreter and bytecode on all fixtures, plus a published benchmark.

### B11. Make it usable by someone else

- **WebAssembly playground:** edit a `.storm` file in the browser, see unit errors as you type, plot the trajectory, drag a parameter. This is the single most effective way to show the project to another person.
- **Language server:** hover any subexpression to see its dimension, and after A1, hover a parameter to see the dimension of its gradient.
- **Real data:** every result so far is synthetic. Fit one real system and publish the residuals. Candidates: a phone-accelerometer recording of a mass on a spring, planetary positions from JPL Horizons (recover the Sun's `GM`), or a public epidemic curve.
- **Comparison table:** gradient accuracy and wall time against SciPy and one differentiable-solver library on the same model, stated with step sizes and tolerances.

### B12. Depth for the flagship aurora demo

A3's dipole demo becomes far more convincing with numbers attached:

- Størmer's conserved canonical angular momentum defines **forbidden regions** the particle can never enter. Plot them and assert no trajectory crosses the boundary.
- The three periodic motions (gyration, bounce between mirror points, azimuthal drift) have known approximate periods. Assert the simulated ratios.
- Show that plain RK4 loses the trapping over long runs while the Boris pusher keeps it.

## Suggested order

1. **A1** dimension-typed gradients, with **B2** `verify` and **B9** language growth done alongside as needed.
2. **A2** nondimensionalization, then **B4** experiment design.
3. **B1** adaptive stepping.
4. **A3** invariants with **B12** as its acceptance evidence, then **B6** because the integrator already exists.
5. **A4** events.
6. **B3** stiffness, **B5** structural identifiability, **B7** discovery, in whichever order stays interesting.
7. **A5** DAEs and **B8** unknown terms as stretch goals.

**B11** is not last. A playground and one real-data fit are worth doing as soon as A1 and A2 work, because they are what makes the project legible to someone who has not read the code.

## If only three things get built

1. **A1 + A2 + B4** as one story: gradients with units, a report of what the data can and cannot determine, and a proposed experiment that fixes it.
2. **A3 + B12:** the aurora problem, with forbidden regions as a hard test.
3. **B11:** the browser playground and one real data set.

## Reading

- Hairer, Lubich, Wanner, *Geometric Numerical Integration* (symplectic methods, Störmer-Verlet).
- Hairer, Nørsett, Wanner, *Solving Ordinary Differential Equations I* (embedded pairs, dense output).
- Griewank and Walther, *Evaluating Derivatives* (AD, checkpointing).
- Chen et al., "Neural Ordinary Differential Equations" (the adjoint versus discretize-then-optimize trade-off).
- Transtrum et al., "Perspective: Sloppiness and emergent theories in physics, biology, and beyond".
- Raue et al., "Structural and practical identifiability analysis ... by exploiting the profile likelihood".
- Brunton, Proctor, Kutz, "Discovering governing equations from data by sparse identification of nonlinear dynamical systems".
- Boris, "Relativistic plasma simulation: optimization of a hybrid code"; Qin et al., "Why is Boris algorithm so good?".
- Størmer, *The Polar Aurora*.
- Pantelides, "The consistent initialization of differential-algebraic systems".
