# MobiusLoop

MobiusLoop is a reactive application template for building modern `egui` applications with `egui_mobius`. 

There is often a need to have a shell or logger or terminal type functionality in an egui application, and having a well tested and versatile widget that is easily customizable facilitates rapid design development with reliable performance. 

# Requirements 
The requirements are well established to define the expectations 
for the widget. 

1. Should be able to be inserted into any `egui` with `eframe` application 
2. Should have a minimal amount of arguments in it's API to interface to the existing GUI application 
3. By consuming a message data structure, will `react` to changes by applying a handler function and then post the resulting text into the shell.  

## Features

Features are enabled by using the `Dynamic<T>` from the crate `egui_mobius_reactive.`

- Chained reactivity can be done with `Derived<T>`
- Thread safe operation, underlying data types are Arc<Mutex<T>>
- Can be easily modified to make your own customized ReShell widget 

## Usage
Add `ReShell` to your `egui` application:
```rust
use reshell::TerminalWidget;

let terminal_widget = TerminalWidget::new();
```
Generally there will be a Dynamic<T> on the app state that can be modified in the main UiApp code. 
See the example `dock_reshell` for an example of updating the state of the TerminalWidget. 

Also take a look at the code in the lib for reshell itself, and you will see how compact the 
codebase is for the widget. 
