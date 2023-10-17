//! Representation of Bevy-Behavior-Tree save data type.
//!
//! Unlike BevyBT Editor, this module is defined in `bevy-bt-core`, which is licensed under MIT. You
//! can implement your own editor which generates compatible data type with BevyBT

use std::collections::HashMap;

use indexmap::IndexMap;

/// BevyBT project structure. The whole directory structure is loaded into this struct.
pub struct Workspace {
    trees: HashMap<String, BehaviorTree>,
    task_defs: HashMap<String, TaskDefinition>,
}

pub struct BehaviorTree {
    root: Node,
    blackboard: HashMap<String, Argument>,
}

pub enum Node {
    Sequence(SequenceNode),
    Selector(SelectorNode),

    /// Instantiation of single action
    Task(TaskDefinition),

    /// Reference to another tree in relative file system.
    TreeRef(TreeRef),
}

/// Blackboard type and its default value representation
#[derive(Debug, serde::Serialize, serde::Deserialize, strum::EnumDiscriminants)]
#[strum_discriminants(derive(serde::Serialize, serde::Deserialize))]
pub enum Argument {
    Boolean(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    UUID(u128),
    F32(f32),
    F64(f64),
    String(String),
    FixedArray(Vec<Argument>),
    DynamicArray(ArgumentDiscriminants, Option<usize>),

    /// Will be generated as [`Option<Box<dyn std::any::Any + Send>>`]
    BoxedAny,
}

pub struct SequenceNode {
    children: Vec<Node>,
}

pub struct SelectorNode {
    random: bool,
    children: Vec<Node>,
}

pub struct TaskNode {}

pub struct TreeRef {
    /// Must be handled properly by behavior tree editor!
    pub relative_path: String,
}

pub struct Parallel {
    /// Only result of the main node is evaluated. Once main node is completed, all other parallel
    /// nodes are aborted
    pub main: Node,

    /// Every node runs in parallel
    pub subs: Vec<Node>,
}

pub struct Task {
    pub definition: TaskDefinition,
}

/// Action definition.
pub struct TaskDefinition {
    /// Input blackboard arguments and their types
    pub inputs: IndexMap<String, Argument>,

    /// Output blackboard arguments and their types
    pub outputs: IndexMap<String, Argument>,
}
