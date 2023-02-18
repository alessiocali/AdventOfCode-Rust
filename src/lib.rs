pub fn clamp<T>(num: T, min: T, max: T) -> T
where T: Ord
{
    std::cmp::max(std::cmp::min(num, max), min)
}