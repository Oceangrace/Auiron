use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::PreviewVisual;
use crate::modules::interfaces::layout::GroupFlow;

pub(crate) struct RowLayout;

impl RowLayout {
    pub(crate) fn apply(node: &WidgetNode, count: usize, index: usize, visual: &mut PreviewVisual) {
        let block_width = node
            .children
            .iter()
            .map(|child| child.layout.x + child.layout.width * child.scale())
            .fold(8.0, f64::max)
            - node
                .children
                .iter()
                .map(|child| child.layout.x)
                .fold(0.0, f64::min);

        let block_height = node
            .children
            .iter()
            .map(|child| child.layout.y + child.layout.height * child.scale())
            .fold(8.0, f64::max)
            - node
                .children
                .iter()
                .map(|child| child.layout.y)
                .fold(0.0, f64::min);

        let (x, y) = match node.group.flow {
            GroupFlow::Row => (index as f64 * (block_width + node.group.gap), 0.0),

            GroupFlow::Grid => (
                (index as f64 % node.group.columns) * (block_width + node.group.gap),
                (index as f64 / node.group.columns).floor() * (block_height + node.group.gap),
            ),
            _ => (0.0, index as f64 * (block_height + node.group.gap)),
        };

        let padding = node.group.padding;

        let width = node.layout.width - padding * 2.0;

        let header = if node.group.header_enabled {
            node.group.header_height
        } else {
            0.0
        };

        let height = node.layout.height - padding * 2.0 - header;

        let across = node.group.columns.min(count as f64);

        let lines = (count as f64 / node.group.columns).ceil();

        let (total_width, total_height) = match node.group.flow {
            GroupFlow::Row => (
                count as f64 * (block_width + node.group.gap) - node.group.gap,
                0.0,
            ),
            GroupFlow::Grid => (
                across * (block_width + node.group.gap) - node.group.gap,
                lines * (block_height + node.group.gap) - node.group.gap,
            ),
            _ => (
                0.0,
                count as f64 * (block_height + node.group.gap) - node.group.gap,
            ),
        };

        visual.offset_x += x + if total_width > 0.0 {
            node.group.grow.offset(true, width, total_width)
        } else {
            0.0
        };
        visual.offset_y += y + if total_height > 0.0 {
            node.group.grow.offset(false, height, total_height)
        } else {
            0.0
        };
    }
}
