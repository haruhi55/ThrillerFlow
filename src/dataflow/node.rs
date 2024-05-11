use std::rc::Rc;

use crate::dataflow::{ThrillerBlock, ThrillerEdge};
use crate::task::Task;
use crate::{next_id, Buffer, ThrillerResult};

/// `ThrillerNodeInnrer` is an enum to represent either an operation or a block.
#[allow(dead_code)]
pub enum ThrillerNodeInner {
    /// An operation.
    Op(Box<dyn Task>),
    /// A buffer.
    Buffer(Rc<Buffer>),
    /// A subgraph block.
    Block(Rc<ThrillerBlock>),
}

/// A Thriller Dataflow Node that represents either a block of subgraph or an operation.
#[allow(dead_code)]
pub struct ThrillerNode {
    inner: Box<ThrillerNodeInner>,
    in_edges: Vec<Rc<ThrillerEdge>>,
    out_edges: Vec<Rc<ThrillerEdge>>,
    predecessors: Vec<Rc<ThrillerNode>>,
    successors: Vec<Rc<ThrillerNode>>,
    id: usize,
    in_degrees: usize,
}

impl ThrillerNode {
    /// Create a new `ThrillerNode` with the given inner type.
    pub fn new(inner: ThrillerNodeInner) -> Self {
        ThrillerNode {
            inner: Box::new(inner),
            in_edges: Vec::new(),
            out_edges: Vec::new(),
            predecessors: Vec::new(),
            successors: Vec::new(),
            id: next_id(),
            in_degrees: 0,
        }
    }

    pub(crate) fn get_name(&self) -> String {
        match self.inner.as_ref() {
            ThrillerNodeInner::Buffer(buffer) => buffer.get_name().clone(),
            _ => panic!("Node is not a buffer"),
        }
    }

    pub(crate) fn get_id(&self) -> usize {
        self.id
    }

    pub(crate) fn get_in_degrees(&self) -> usize {
        self.in_degrees
    }

    pub(crate) fn get_successors(&self) -> &Vec<Rc<ThrillerNode>> {
        &self.successors
    }

    pub(crate) fn get_inner(&self) -> &ThrillerNodeInner {
        &self.inner
    }
}

impl Task for ThrillerNode {
    fn emit(&self) -> ThrillerResult<String> {
        match self.inner.as_ref() {
            ThrillerNodeInner::Op(task) => task.emit(),
            _ => panic!("Node is not an operation"),
        }
    }
}
