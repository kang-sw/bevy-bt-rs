//! Representation of Bevy-Behavior-Tree save data type.
//!
//! Unlike BevyBT Editor, this module is defined in `bevy-bt-core`, which is licensed under MIT. You
//! can implement your own editor which generates compatible data type with BevyBT

use std::collections::HashMap;

use indexmap::IndexMap;

/// BevyBT project structure. The whole directory structure is loaded into this struct.
///
/// - TODO: Workspace directory manipulation functionalities
///     - `write_tree`: Validates tree implementation and write tree and corresponding tasks to file
///       system
///         - Validates if tree's every [`TreeRef`] points valid tree path.
///         - Validates if there's no conflicting [`UserAction`] definitions.
///     - `reload`: Reload workspace from file system
pub struct Workspace {
    trees: HashMap<String, BehaviorTree>,
    user_actions: HashMap<String, UserAction>,
    user_structs: HashMap<String, UserStruct>,
}

pub struct BehaviorTree {
    /// The root node, which is entry point of execution hierarchy.
    root: Node,

    /// Floating nodes that does not belong to any parent node.
    floating_nodes: Vec<Node>,

    /// Blackboard variable definitions
    blackboard: HashMap<String, Argument>,
}

pub struct Node {
    /// Overrided title of this node.
    pub title: String,

    /// Decorator of this node
    pub decorators: Vec<Decorator>,

    /// Position in grid
    pub position: [f64; 2],

    /// Comment of this node
    pub comment: String,

    /// Actual node class
    pub class: NodeClass,
}

pub enum NodeClass {
    Sequence(SequenceNode),
    Selector(SelectorNode),
    Decorator(Decorator),

    /// Instantiation of single action
    Task(Task),

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
    FixedArray(Box<Argument>, usize),
    Tuple(Vec<Argument>),
    DynamicArray(ArgumentDiscriminants, Option<usize>),

    /// Will be generated as [`Option<Box<dyn std::any::Any + Send>>`]
    BoxedAny,

    /// Handle to Bevy entity
    Entity,

    /// Path to user defined struct
    UserStruct(String),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserStruct {
    pub fields: IndexMap<String, Argument>,
}

pub struct SequenceNode {
    pub children: Vec<Node>,
}

pub struct SelectorNode {
    pub random: bool,
    pub children: Vec<Node>,
}

pub struct TaskNode {}

pub struct TreeRef {
    /// Must be handled properly by behavior tree editor!
    pub relative_path: String,
}

pub struct Parallel {
    /// Only result of the main node is evaluated. Once main node is completed, all other parallel
    /// nodes are aborted
    pub main: NodeClass,

    /// Every node runs in parallel
    pub subs: Vec<NodeClass>,
}

pub enum Task {
    UserAction(UserAction),

    WaitFor {
        timeout: f64,
    },

    /// Wait forever before any interrupt
    WaitForever,
}

pub struct UserAction {
    inputs: Vec<Argument>,
    outputs: Vec<Argument>,
}

pub enum Decorator {
    /// TODO: Blackboard evaluation expression

    /// Check if blackboard argument of given name equals with given value. Type must be same.
    BlackboardEquals(String, Argument),

    /// For boolean, non-false. For numerics, non-zero. For strings or references, non-empty.
    BlackboardNonEmpty(String),

    /// Expects user returns true or false; if user action executer returns [`None`], it'll
    /// use specified boolean value as default.
    ///
    /// This differs from plain [`UserAction`] task, which is
    UserEval { logic_name: String, inputs: IndexMap<String, Argument>, default: bool },
}
