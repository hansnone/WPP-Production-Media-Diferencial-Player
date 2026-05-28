//! Marcadores in/out para informes y saltos en timeline (dominio puro).

use serde::{Deserialize, Serialize};

/// Marcador de usuario en segundos (PTS).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marcador {
    pub id: u64,
    pub etiqueta: String,
    pub pts_in: f64,
    pub pts_out: Option<f64>,
    pub color_rgb: [u8; 3],
}

impl Marcador {
    #[must_use]
    pub fn nuevo(id: u64, etiqueta: impl Into<String>, pts_in: f64) -> Self {
        Self {
            id,
            etiqueta: etiqueta.into(),
            pts_in,
            pts_out: None,
            color_rgb: [42, 143, 232],
        }
    }

    /// Duración del tramo marcado; si no hay out, devuelve 0.
    #[must_use]
    pub fn duracion(&self) -> f64 {
        self.pts_out
            .map(|out| (out - self.pts_in).max(0.0))
            .unwrap_or(0.0)
    }

    /// Si `pts` cae dentro del rango [in, out] (out abierto si es None).
    #[must_use]
    pub fn contiene_pts(&self, pts: f64) -> bool {
        if pts < self.pts_in {
            return false;
        }
        match self.pts_out {
            Some(out) => pts <= out,
            None => true,
        }
    }
}

/// Colección ordenada por `pts_in`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListaMarcadores {
    items: Vec<Marcador>,
}

impl ListaMarcadores {
    #[must_use]
    pub fn vacia() -> Self {
        Self { items: Vec::new() }
    }

    pub fn agregar(&mut self, marcador: Marcador) {
        self.items.push(marcador);
        self.items.sort_by(|a, b| {
            a.pts_in
                .partial_cmp(&b.pts_in)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    pub fn eliminar(&mut self, id: u64) -> bool {
        if let Some(pos) = self.items.iter().position(|m| m.id == id) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = &Marcador> {
        self.items.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Marcador activo en el PTS dado, si hay solapamiento.
    #[must_use]
    pub fn en_pts(&self, pts: f64) -> Option<&Marcador> {
        self.items.iter().find(|m| m.contiene_pts(pts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duracion_con_out() {
        let m = Marcador {
            id: 1,
            etiqueta: "a".into(),
            pts_in: 1.0,
            pts_out: Some(3.0),
            color_rgb: [0, 0, 0],
        };
        assert!((m.duracion() - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn lista_ordenada_y_busqueda() {
        let mut lista = ListaMarcadores::vacia();
        lista.agregar(Marcador::nuevo(2, "b", 5.0));
        lista.agregar(Marcador::nuevo(1, "a", 1.0));
        assert_eq!(lista.iter().next().unwrap().id, 1);
        assert!(lista.en_pts(1.5).is_some());
        assert!(lista.eliminar(1));
        assert_eq!(lista.len(), 1);
        assert!(lista.eliminar(2));
        assert!(lista.is_empty());
    }
}
