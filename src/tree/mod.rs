//! Contains both [a high-level interface to Taffy](crate::Taffy) using a ready-made node tree, and [a trait for defining a custom node trees](crate::tree::LayoutTree) / utility types to help with that.

use crate::geometry::{AbsoluteAxis, Line, Size};
use crate::style::{AvailableSpace, Style};

// Submodules
mod cache;
pub use cache::Cache;
mod node;
#[cfg(feature = "taffy_tree")]
use node::NodeData;
pub use node::NodeId;
#[cfg(feature = "taffy_tree")]
mod taffy_tree;
#[cfg(feature = "taffy_tree")]
pub use taffy_tree::{Taffy, TaffyError, TaffyResult};
mod layout;
pub use layout::{CollapsibleMarginSet, Layout, LayoutInput, LayoutOutput, RequestedAxis, RunMode, SizingMode};

/// This if the core abstraction in Taffy. Any type that *correctly* implements `PartialLayoutTree` can be laid out using Taffy's algorithms.
///
/// The type implementing `PartialLayoutTree` would typically be an entire tree of nodes (or a view over an entire tree of nodes).
/// However, `PartialLayoutTree` and Taffy's algorithm implementations have been designed such that they can be used for a laying out a single
/// node that only has access to it's immediate children.
pub trait PartialLayoutTree {
    /// Type representing an iterator of the children of a node
    type ChildIter<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;

    /// Get the list of children IDs for the given node
    fn child_ids(&self, parent_node_id: NodeId) -> Self::ChildIter<'_>;

    /// Get the number of children for the given node
    fn child_count(&self, parent_node_id: NodeId) -> usize;

    /// Get a specific child of a node, where the index represents the nth child
    fn get_child_id(&self, parent_node_id: NodeId, child_index: usize) -> NodeId;

    /// Get a reference to the [`Style`] for this node.
    fn get_style(&self, node_id: NodeId) -> &Style;

    /// Get a mutable reference to the node's unrounded layout
    fn get_unrounded_layout_mut(&mut self, node_id: NodeId) -> &mut Layout;

    /// Get a mutable reference to the [`Cache`] for this node.
    fn get_cache_mut(&mut self, node_id: NodeId) -> &mut Cache;

    /// Compute the specified node's size or full layout given the specified constraints
    fn compute_child_layout(&mut self, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput;
}

/// Extends [`PartialLayoutTree`] with an additional guarantee: that the child/children methods can be used to recurse
/// infinitely down the tree. Enables Taffy's rounding and debug printing methods to be used.
pub trait LayoutTree: PartialLayoutTree {
    /// Get a reference to the node's final layout
    fn get_final_layout(&self, node_id: NodeId) -> &Layout;
    /// Get a mutable reference to the node's final layout
    fn get_final_layout_mut(&mut self, node_id: NodeId) -> &mut Layout;
}

/// A private trait which allows us to add extra convenience methods to types which implement
/// LayoutTree without making those methods public.
pub(crate) trait PartialLayoutTreeExt: PartialLayoutTree {
    /// Compute the size of the node given the specified constraints
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn measure_child_size(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
        axis: AbsoluteAxis,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> f32 {
        self.compute_child_layout(
            node_id,
            LayoutInput {
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                axis: axis.into(),
                run_mode: RunMode::ComputeSize,
                vertical_margins_are_collapsible,
            },
        )
        .size
        .get_abs(axis)
    }

    /// Perform a full layout on the node given the specified constraints
    #[inline(always)]
    fn perform_child_layout(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> LayoutOutput {
        self.compute_child_layout(
            node_id,
            LayoutInput {
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                axis: RequestedAxis::Both,
                run_mode: RunMode::PerformLayout,
                vertical_margins_are_collapsible,
            },
        )
    }
}

impl<T: PartialLayoutTree> PartialLayoutTreeExt for T {}
