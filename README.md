# Castel
An exploration in UI and interface.

## Current Target: Castel-Wiki
A cross-platform desktop application to create and manage notes on interlinked topics.

## Goals

* Search-oriented browsing
* Quick jumping interface
* Blank page creation
	* Outline (TOC) mode
* Spot editor
* Link completion

## Anti-Goals

* Plain-text markup or file format
* In-line display of media

## Things to consider

* GUI
	* What library tool are we going to use
		* Start with basic libraries for window creation, opengl bindings, and font rasterizing and roll everything ourselves
		* Use the best/only native Rust GUI option, which is GTK3, leaving very little in the way of customization
		* Pick a different C/C++ library and use Rust's bindgen to connect to it
	* how much of this is going to be from scratch
	* What is the GUI going to look like
		* have as little of the tools obscure the edit page
		* keep buttons on the side
		* what buttons/shortcuts will we need
* Back end
	* SQLite db for storing everything - Rust has an ORM to deal with this
		* Best models for what we want to do
		* get the n00bs some SQL/db design experience
	* Hyperlinking between pages/media ?? 
		* How, what's our data structure going to look like - keep away from markup, at least not that the user can see
		* how will hyperlink completion work on the front end, back end can easily store page id numbers
		* unique page names??
