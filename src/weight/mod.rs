pub fn get_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

#[cfg(test)]
mod tests {
    use super::get_on_mars;

    #[test]
    fn it_should_get_on_mars() {
        let result = get_on_mars(100.0);
        assert_eq!(result, 37.828747);
    }
}
