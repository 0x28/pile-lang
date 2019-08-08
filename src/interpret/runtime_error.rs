pub fn ensure_element<T>(stack: &mut Vec<T>) -> Result<T, String> {
    stack.pop().ok_or_else(|| "Stack underflow".to_owned())
}
