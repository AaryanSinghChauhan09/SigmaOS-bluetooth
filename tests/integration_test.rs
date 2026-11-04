use anyhow::Result;
use sigmaos_component::{Config, bluetooth};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default_aaa() {
        // Arrange & Act
        let config = Config::default();
        // Assert
        assert!(config.enabled);
        assert!(!config.debug);
    }

    #[test]
    fn test_bluetooth_start_stop_aaa() -> Result<()> {
        // Arrange
        let config = Config::default();
        let mut bt = bluetooth::new(config);
        // Act
        bt.initialize()?;
        bt.start()?;
        // Assert
        bt.stop()?;
        Ok(())
    }
}
