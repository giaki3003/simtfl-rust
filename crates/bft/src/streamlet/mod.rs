// bft/src/streamlet/lib.rs

use crate::*; // Import everything from the parent module (`bft/src/lib.rs`)

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamletGenesis {
    pub n: usize,
    pub t: usize,
}

impl StreamletGenesis {
    pub fn new(n: usize) -> Self {
        let t = (n * 2 + 2) / 3;
        Self { n, t }
    }

    pub fn last_final(&self) -> PermissionedBFTEnum {
        PermissionedBFTEnum::Base(PermissionedBFTBase {
            n: self.n,
            t: self.t,
            parent: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamletProposal {
    pub parent: Box<PermissionedBFTEnum>,
    pub epoch: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamletBlock {
    pub proposal: Box<StreamletProposal>,
    pub parent: Option<Box<PermissionedBFTEnum>>,
}

impl StreamletProposal {
    pub fn new(parent: Box<PermissionedBFTEnum>, epoch: usize) -> Self {
        // Match on the PermissionedBFTEnum variant to access its methods
        let parent_epoch = match *parent {
            PermissionedBFTEnum::Base(ref base) => base.epoch(),
            PermissionedBFTEnum::Block(ref block) => block.proposal.epoch(),
            PermissionedBFTEnum::Proposal(ref proposal) => proposal.epoch(),
        };

        // Ensure the new epoch is greater than the parent's epoch
        assert!(epoch > parent_epoch);

        logging::log_info(&format!(
            "Creating StreamletProposal with epoch {} and parent {:?}",
            epoch, parent
        ));

        Self { parent, epoch }
    }

    pub fn n(&self) -> usize {
        self.parent.n()
    }

    pub fn t(&self) -> usize {
        self.parent.t()
    }

    pub fn epoch(&self) -> usize {
        self.epoch
    }
}

impl StreamletBlock {
    pub fn last_final(&self) -> PermissionedBFTEnum {
        logging::log_info("Calculating last_final for StreamletBlock.");
        let mut last = self;
        let mut middle: Option<&StreamletBlock> = None;
        let mut first: Option<&StreamletBlock> = None;

        while let Some(parent) = last.parent.as_ref() {
            match **parent {
                PermissionedBFTEnum::Block(ref block) => {
                    if let (Some(first_block), Some(middle_block)) = (first, middle) {
                        if first_block.epoch() + 1 == middle_block.epoch()
                            && middle_block.epoch() + 1 == last.epoch()
                        {
                            return PermissionedBFTEnum::Block(middle_block.clone());
                        }
                    }

                    first = Some(middle.unwrap_or(last));
                    middle = Some(last);
                    last = block;
                }
                _ => break,
            }
        }

        if let Some(middle_block) = middle {
            return PermissionedBFTEnum::Block(middle_block.clone());
        }

        // Match on the PermissionedBFTEnum variant to access its methods
        PermissionedBFTEnum::Base(PermissionedBFTBase {
            n: match *self.proposal.parent {
                PermissionedBFTEnum::Base(ref base) => base.n,
                PermissionedBFTEnum::Block(ref block) => block.proposal.n(),
                PermissionedBFTEnum::Proposal(ref proposal) => proposal.n(),
            },
            t: match *self.proposal.parent {
                PermissionedBFTEnum::Base(ref base) => base.t,
                PermissionedBFTEnum::Block(ref block) => block.proposal.t(),
                PermissionedBFTEnum::Proposal(ref proposal) => proposal.t(),
            },
            parent: None,
        })
    }

    pub fn epoch(&self) -> usize {
        self.proposal.epoch()
    }
}
