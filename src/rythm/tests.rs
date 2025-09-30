#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::waves;

    #[test]
    fn common_use() {
        let x = RythmBuilder::default()
            .with_tempo_bpm(90.0)
            .finalize()
            .unwrap();
        x.hit(0.25, waves::SineBuilder::default().finalize().unwrap());
    }

    #[test]
    fn test_macro_attr() {
        let x: RythmBuilder<waves::Sine> = RythmBuilder {
            tempo_bpm: 60.0,
            rythm: vec![],
        }
        .with_tempo_bpm(45.0);
        // .with_rythm(vec![1.0]);
        assert_eq!(x.get_tempo_bpm(), &45.0);
    }
}
