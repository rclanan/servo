/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// Style retrieval from DOM elements.

use css::node_util::NodeUtil;
use incremental::RestyleDamage;
use wrapper::ThreadSafeLayoutNode;

use style::ComputedValues;
use sync::Arc;

/// Node mixin providing `style` method that returns a `NodeStyle`
pub trait StyledNode {
    fn style<'a>(&'a self) -> &'a Arc<ComputedValues>;
    fn unstyle(self);
    fn restyle_damage(self) -> RestyleDamage;
    fn set_restyle_damage(self, damage: RestyleDamage);
}

impl<'ln> StyledNode for ThreadSafeLayoutNode<'ln> {
    #[inline]
    fn style<'a>(&'a self) -> &'a Arc<ComputedValues> {
        self.get_css_select_results()
    }

    fn unstyle(self) {
        self.remove_css_select_results()
    }

    fn restyle_damage(self) -> RestyleDamage {
        self.get_restyle_damage()
    }

    fn set_restyle_damage(self, damage: RestyleDamage) {
        fn doit<N: NodeUtil>(n: N, damage: RestyleDamage) {
            n.set_restyle_damage(damage);
        }
        doit(self, damage);
    }
}
