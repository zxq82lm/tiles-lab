# tiles-lab

## Definitions
- A **tile** is the generic building block of the tree: it can be a **pane** or a **container**.  
- A **pane** is a leaf with content; its title shows up and is editable.  
- A **container** groups tiles and may be of type *Tabs* (tabbed views), *Linear* (horizontal/vertical split), or *Grid*.

## Context
Container titles are normally hidden. They appear only if the container is inside a *Tabs*, in which case they serve as tab labels. Default labels (“Horizontal” / “Vertical” for a Linear) are auto-generated and not editable.

## Layout structure

The tree built in `App::new` looks like this:
```
Tabs
├── Linear (Horizontal)
│ ├── Pane: "H: Left"
│ └── Pane: "H: Right"
└── Linear (Vertical)
├── Pane: "V: Top"
├── Pane: "V: Middle"
└── Pane: "V: Bottom"
```