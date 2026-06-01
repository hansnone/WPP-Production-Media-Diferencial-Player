//! Eventos y notas de control de calidad (M10): modelo de dominio y registro en memoria.

use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Origen del hallazgo QC (filtros en UI y reportes futuros).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TipoEventoQc {
    Manual,
    Video,
    Audio,
}

impl TipoEventoQc {
    #[must_use]
    pub fn etiqueta(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::Video => "video",
            Self::Audio => "audio",
        }
    }
}

/// Nota de texto asociada a un evento (puede anclarse a otro PTS).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotaQc {
    pub id: u64,
    pub texto: String,
    pub pts_secs: f64,
    pub creado_unix_ms: i64,
}

/// Hallazgo QC en un instante de la línea de tiempo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventoQc {
    pub id: u64,
    pub tipo: TipoEventoQc,
    pub pts_secs: f64,
    pub titulo: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descripcion: Option<String>,
    #[serde(default)]
    pub notas: Vec<NotaQc>,
    pub creado_unix_ms: i64,
}

/// Colección persistible de eventos para un par de fuentes A/B.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistroEventosQc {
    /// Identificador estable del proyecto (hash de rutas A+B).
    pub clave_proyecto: String,
    pub eventos: Vec<EventoQc>,
    /// Próximo id libre para eventos y notas.
    pub siguiente_id: u64,
}

impl Default for RegistroEventosQc {
    fn default() -> Self {
        Self::vacio("sin-proyecto")
    }
}

impl RegistroEventosQc {
    #[must_use]
    pub fn vacio(clave_proyecto: impl Into<String>) -> Self {
        Self {
            clave_proyecto: clave_proyecto.into(),
            eventos: Vec::new(),
            siguiente_id: 1,
        }
    }

    /// Clave de disco a partir de rutas absolutas de A y B (vacías → sin-proyecto).
    #[must_use]
    pub fn clave_desde_rutas(ruta_a: Option<&str>, ruta_b: Option<&str>) -> String {
        let a = ruta_a.unwrap_or("").trim();
        let b = ruta_b.unwrap_or("").trim();
        if a.is_empty() && b.is_empty() {
            return "sin-proyecto".into();
        }
        let mut hasher = DefaultHasher::new();
        a.hash(&mut hasher);
        b.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }

    fn reservar_id(&mut self) -> u64 {
        let id = self.siguiente_id;
        self.siguiente_id = self.siguiente_id.saturating_add(1);
        id
    }

    /// Añade un evento y lo devuelve (ordenado por PTS).
    pub fn agregar_evento(
        &mut self,
        tipo: TipoEventoQc,
        pts_secs: f64,
        titulo: impl Into<String>,
        descripcion: Option<String>,
        ahora_unix_ms: i64,
    ) -> EventoQc {
        let evento = EventoQc {
            id: self.reservar_id(),
            tipo,
            pts_secs: pts_secs.max(0.0),
            titulo: titulo.into(),
            descripcion,
            notas: Vec::new(),
            creado_unix_ms: ahora_unix_ms,
        };
        self.eventos.push(evento.clone());
        self.ordenar_por_pts();
        evento
    }

    pub fn eliminar_evento(&mut self, id: u64) -> bool {
        if let Some(pos) = self.eventos.iter().position(|e| e.id == id) {
            self.eventos.remove(pos);
            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn obtener(&self, id: u64) -> Option<&EventoQc> {
        self.eventos.iter().find(|e| e.id == id)
    }

    #[must_use]
    pub fn obtener_mut(&mut self, id: u64) -> Option<&mut EventoQc> {
        self.eventos.iter_mut().find(|e| e.id == id)
    }

    /// Añade nota a un evento existente.
    pub fn agregar_nota(
        &mut self,
        evento_id: u64,
        texto: impl Into<String>,
        pts_secs: f64,
        ahora_unix_ms: i64,
    ) -> Option<NotaQc> {
        let id = self.reservar_id();
        let nota = NotaQc {
            id,
            texto: texto.into(),
            pts_secs: pts_secs.max(0.0),
            creado_unix_ms: ahora_unix_ms,
        };
        let evento = self.obtener_mut(evento_id)?;
        evento.notas.push(nota.clone());
        Some(nota)
    }

    /// Lista ordenada por PTS, con filtro opcional por tipo.
    #[must_use]
    pub fn listar(&self, filtro_tipo: Option<TipoEventoQc>) -> Vec<&EventoQc> {
        self.eventos
            .iter()
            .filter(|e| filtro_tipo.map_or(true, |t| e.tipo == t))
            .collect()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.eventos.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.eventos.is_empty()
    }

    fn ordenar_por_pts(&mut self) {
        self.eventos.sort_by(|a, b| {
            a.pts_secs
                .partial_cmp(&b.pts_secs)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.id.cmp(&b.id))
        });
    }
}

/// Serializa el registro a JSON legible.
pub fn serializar_registro(registro: &RegistroEventosQc) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(registro)
}

/// Carga desde JSON; si falla el parseo, devuelve registro vacío con la clave pedida.
pub fn deserializar_registro(json: &str, clave_fallback: &str) -> RegistroEventosQc {
    match serde_json::from_str::<RegistroEventosQc>(json) {
        Ok(mut r) => {
            if r.clave_proyecto.is_empty() {
                r.clave_proyecto = clave_fallback.into();
            }
            r.ordenar_por_pts();
            r
        }
        Err(_) => RegistroEventosQc::vacio(clave_fallback),
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn clave_estable_para_mismas_rutas() {
        let k1 = RegistroEventosQc::clave_desde_rutas(Some("/a.mov"), Some("/b.mov"));
        let k2 = RegistroEventosQc::clave_desde_rutas(Some("/a.mov"), Some("/b.mov"));
        assert_eq!(k1, k2);
        assert_ne!(k1, "sin-proyecto");
    }

    #[test]
    fn agregar_nota_y_filtrar() {
        let mut reg = RegistroEventosQc::vacio("test");
        let e = reg.agregar_evento(
            TipoEventoQc::Video,
            12.5,
            "Caída SSIM",
            None,
            1_000,
        );
        assert!(reg
            .agregar_nota(e.id, "Revisar GOP", 12.5, 2_000)
            .is_some());
        assert_eq!(reg.listar(Some(TipoEventoQc::Video)).len(), 1);
        assert!(reg.listar(Some(TipoEventoQc::Manual)).is_empty());
        assert!(reg.eliminar_evento(e.id));
        assert!(reg.is_empty());
    }

    #[test]
    fn roundtrip_json() {
        let mut reg = RegistroEventosQc::vacio("abc");
        reg.agregar_evento(TipoEventoQc::Manual, 0.0, "Inicio", None, 0);
        let json = serializar_registro(&reg).unwrap();
        let loaded = deserializar_registro(&json, "abc");
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded.clave_proyecto, "abc");
    }
}
