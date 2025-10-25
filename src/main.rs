mod iommu;
mod gpu;
fn main() -> std::io::Result<()> {
    iommu::list_iommu_groups()?;
    Ok(())
}
