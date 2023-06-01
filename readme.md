# 2048 

第一步
使用一个Vec<Vec<u32>>来表示棋盘，其中每个元素表示棋盘上的一个格子，0表示空格子，其他数字表示格子上的数字。
再需要一个分数Score，用来记录当前的分数。
需要一个移动次数Moves，用来记录当前的移动次数。
叫棋盘

```rust
	struct Panel {
		valueSave: Vec<Vec<u32>>,
		score: u32,
		moves: u32
	}

```
init 初始化棋盘，将棋盘上的所有格子都设置为0，分数和移动次数都设置为0。
使用一个Resource保存棋盘的状态，包括棋盘上的数字，分数和移动次数。
将画面初始化 使用boxmesh来设置 颜色样式
```rust
// 棋盘面板
commands.spawn(MaterialMesh2dBundle {
		mesh: meshes.add(shape::Box::new(WINDOW_HEIGHT, WINDOW_HEIGHT, 0.0).into()).into(),
		material: materials.add(ColorMaterial::from(COLOR_BACKGROUND)),
		transform: Transform::from_xyz((WINDOW_WIDTH - WINDOW_HEIGHT) / 2.0, 0.0, 0.0),
		..default()
	});
```

同时，需要一个config.rs来存储config等等信息

首先，设置棋盘大小,移动方向等
```rust
pub static WINDOW_WIDTH: f32 = 800.0;
pub static WINDOW_HEIGHT: f32 = 600.0;
pub static CELL_SPACE: f32 = 6.0;
pub static CELL_SIDE_NUM: u32 = 4;

pub enum MOVE_DIRECTION {
	NONE,
	UP,
	DOWN,
	LEFT,
	RIGHT
}

pub enum STATE {
    RUNNING,
    WIN,
    LOSE
}


pub struct CELL_VALUE_SAVE {
	pub(crate) valueSave: Vec<Vec<u32>>,
	pub(crate) cellBackGround: Vec<HandleId>,
	pub(crate) score: u32
    pub(crate) moves: u32
}

pub struct CellValue;

```






