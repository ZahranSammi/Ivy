# Issue: Add a single-command launcher for Ivy

## Summary

Provide one documented command that starts Ivy's Docker services, waits until the
frontend is ready, and opens the application in the default browser.

## Motivation

Starting Ivy currently requires users to remember the service startup order and
manually open the frontend URL. A single launcher reduces setup friction and gives
users clear feedback when Docker or the application is unavailable.

## Acceptance criteria

- `bun start` starts the Docker Compose stack in detached mode and rebuilds images.
- The launcher resolves the repository root so it works when invoked outside the repo.
- The launcher waits for `http://localhost` to become reachable before opening it.
- Missing Docker, Docker startup failures, and readiness timeouts produce actionable
  messages and a non-zero exit status.
- The launcher prints the URL and the command used to stop the stack.
- The root `package.json` exposes the command as the `start` script.

## Scope

This issue covers the local developer launcher and its documentation. It does not
change the application's scan, consent, scope-enforcement, or exploitation behavior.

