//! Possible types of completions.

/// The type of completion attempted. [ref](https://www.gnu.org/software/bash/manual/html_node/Bash-Variables.html#index-COMP_005fTYPE).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CompletionType {
    /// Normal completion.
    Normal = b'\t',
    /// Listing completions after successive tabs.
    List = b'?',
    /// Listing alternatives on partial word completion.
    ListAlternatives = b'!',
    /// List completions if the word is not unmodified.
    ListUnmodified = b'@',
    /// Menu completion.
    Menu = b'%',
}

impl From<CompletionType> for u8 {
    fn from(completion_type: CompletionType) -> Self {
        completion_type as Self
    }
}

impl From<CompletionType> for char {
    fn from(completion_type: CompletionType) -> Self {
        completion_type as u8 as Self
    }
}

impl TryFrom<u8> for CompletionType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            b'\t' => Ok(Self::Normal),
            b'?' => Ok(Self::List),
            b'!' => Ok(Self::ListAlternatives),
            b'@' => Ok(Self::ListUnmodified),
            b'%' => Ok(Self::Menu),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for CompletionType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, ()> {
        match value as u8 {
            b'\t' => Ok(Self::Normal),
            b'?' => Ok(Self::List),
            b'!' => Ok(Self::ListAlternatives),
            b'@' => Ok(Self::ListUnmodified),
            b'%' => Ok(Self::Menu),
            _ => Err(()),
        }
    }
}
