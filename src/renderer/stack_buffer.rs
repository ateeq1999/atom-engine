use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct StackBuffer {
    stacks: IndexMap<String, Vec<String>>,
}

impl StackBuffer {
    pub fn new() -> Self {
        StackBuffer {
            stacks: IndexMap::new(),
        }
    }

    pub fn push(&mut self, name: &str, content: String) {
        self.stacks
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(content);
    }

    pub fn prepend(&mut self, name: &str, content: String) {
        self.stacks
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .insert(0, content);
    }

    pub fn drain(&mut self, name: &str) -> String {
        match self.stacks.remove(name) {
            Some(contents) => contents.join("\n"),
            None => String::new(),
        }
    }

    pub fn peek(&self, name: &str) -> &[String] {
        self.stacks.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn has(&self, name: &str) -> bool {
        self.stacks.contains_key(name)
    }

    pub fn is_empty(&self) -> bool {
        self.stacks.is_empty()
    }

    pub fn clear(&mut self) {
        self.stacks.clear();
    }

    pub fn len(&self) -> usize {
        self.stacks.len()
    }
}

impl Default for StackBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut buffer = StackBuffer::new();
        buffer.push("scripts", "<script>1</script>".to_string());
        buffer.push("scripts", "<script>2</script>".to_string());

        assert_eq!(buffer.peek("scripts").len(), 2);
    }

    #[test]
    fn test_prepend() {
        let mut buffer = StackBuffer::new();
        buffer.push("scripts", "<script>2</script>".to_string());
        buffer.prepend("scripts", "<script>1</script>".to_string());

        let items = buffer.peek("scripts");
        assert_eq!(items[0], "<script>1</script>");
    }

    #[test]
    fn test_drain() {
        let mut buffer = StackBuffer::new();
        buffer.push("scripts", "<script>1</script>".to_string());
        buffer.push("scripts", "<script>2</script>".to_string());

        let drained = buffer.drain("scripts");
        assert_eq!(drained, "<script>1</script>\n<script>2</script>");

        // After drain, stack should be empty
        assert!(buffer.peek("scripts").is_empty());
    }

    #[test]
    fn test_drain_empty_stack() {
        let mut buffer = StackBuffer::new();
        let result = buffer.drain("missing");
        assert!(result.is_empty());
    }
}
