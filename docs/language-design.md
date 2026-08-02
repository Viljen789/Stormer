# Language design goals

This document records semantic goals only.
It does not define final syntax, grammar, file formats, or a stable public interface.

## Declarative dynamical-system models

A model should state physical relationships and evolution equations declaratively.
Users should be able to inspect what quantities exist, how they relate, and which equations determine motion without encoding a solver procedure.

## State variables

State variables represent the quantities whose values evolve during simulation.
The language should distinguish state from derived quantities and parameters so that the runtime has an explicit dynamical system to execute.

## Parameters

Parameters represent model quantities that remain constant during one simulation.
The semantics should distinguish known parameters from unknown parameters selected for calibration.

## Initial conditions

Every state required to begin a simulation should have an explicit and dimensionally valid initial condition.
Missing, conflicting, or underdetermined initial conditions should produce clear diagnostics rather than implicit defaults.

## Differential equations

Differential equations describe how state changes over time.
The language should express ordinary differential-equation models directly while allowing later lowering into a normalized first-order state system.

## Physical dimensions

Physical dimensions are part of the language's semantic type system, not presentation metadata.
Operations and equations should be checked for dimensional consistency before a model reaches numerical execution.
The exact unit notation and conversion rules remain undecided.

## Observations

Observation declarations should connect measurable data to model quantities without embedding data-loading or optimizer behavior in the model semantics.
They should eventually support comparison between simulated and observed trajectories with explicit dimensions and provenance.

## Unknown parameters

A model-calibration workflow needs an explicit way to identify parameters whose values should be estimated from observations.
The design should keep model meaning separate from a particular optimizer or inference algorithm.

## Diagnostics

Diagnostics should explain source locations, relevant model concepts, dimensional expectations, and plausible corrective action.
Invalid models should not be lowered into executable IR merely to continue a workflow.

## Initial language exclusions

The first language version will not include general mutation, an object system, concurrency, or arbitrary I/O.
It is not intended to become a general-purpose programming language.
These constraints leave syntax and representation choices open while focusing semantic design on inspectable physical models.
