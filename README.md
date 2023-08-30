# Panoptes

## Description

Panoptes is a simple Rust application that performs two main functions:

1. It daemonizes itself so it can run in the background.
2. It continually polls the name of the currently active window on your desktop and sends this information to a specified API endpoint (`https://live.ghussein.org/api/desktop`).

## Usage

1. Ensure that the `OPTES_PASSWORD` environment variable is set.
2. Run the application.

The application will daemonize itself and start sending data to the API endpoint at regular intervals defined by `POLLING_DURATION_IN_SECONDS`.
