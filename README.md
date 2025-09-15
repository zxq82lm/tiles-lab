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

## Editable Tab/Container Titles

### `tiles.rs`
- Added `container_titles: HashMap<TileId, String>`.
- New helpers: `get_container_title`, `set_container_title`, `clear_container_title`, `container_title_or_default`.
- Updated `gc_root` to prune unused titles.
- Updated `simplify` to keep containers with custom titles.
- Added `move_container_title_if_any` to transfer custom titles when merging linear containers (untested edge case).

### `behavior.rs`
- `tab_title_for_tile`: now calls `tab_title_for_container` for containers.
- Added `tab_title_for_container`.
- Added `set_container_title`.
- Extended `tab_ui`: double-click to enter rename mode, inline edit with `TextEdit`, commit on Enter/blur, empty clears.

## Install
```
~/workspace/
├─ egui_tiles/ fork, checkout branch `feat/editable-container-titles`
└─ tiles-lab/ # this project
```
- Branch [feat/editable-container-titles](https://github.com/zxq82lm/egui_tiles/tree/feat/editable-container-titles)
