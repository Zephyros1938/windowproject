use nalgebra_glm as glm;

pub fn mat4<T: glm::Number>(n: T) -> glm::TMat4<T> {
    let zero = T::zero();
    glm::TMat4::new(
        n, zero, zero, zero, zero, n, zero, zero, zero, zero, n, zero, zero, zero, zero, n,
    )
}
