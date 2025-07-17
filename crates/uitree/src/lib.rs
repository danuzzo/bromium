
pub type UIHashMap<K, V, S = std::hash::RandomState> = std::collections::HashMap<K, V, S>;
type UIHashSet<T, S = std::hash::RandomState> = std::collections::HashSet<T, S>;

mod macros;

mod tree_map;
use tree_map::UITreeMap;

mod uiexplore;
pub use uiexplore::{UITree, UIElementInTree, get_all_elements};

mod uiexplore_iter;
pub use uiexplore_iter::{UITree as UITreeIter, UIElementInTree as UIElementInTreeIter, get_all_elements_iterative};

mod xml;