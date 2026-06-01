//! Persistencia JSON de eventos QC en el directorio de datos de la app (M10).

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use diffplayerqc_core::{
    deserializar_registro, serializar_registro, EventoQc, RegistroEventosQc, TipoEventoQc,
};
use tauri::{AppHandle, Emitter, Manager};

/// Estado compartido del registro de eventos (un proyecto activo a la vez).
pub struct ServicioEventosQc {
    inner: Mutex<EstadoInterno>,
}

struct EstadoInterno {
    registro: RegistroEventosQc,
    directorio: PathBuf,
    clave_cargada: String,
}

impl ServicioEventosQc {
    pub fn nuevo(directorio: PathBuf) -> Self {
        let _ = fs::create_dir_all(&directorio);
        Self {
            inner: Mutex::new(EstadoInterno {
                registro: RegistroEventosQc::vacio("sin-proyecto"),
                directorio,
                clave_cargada: "sin-proyecto".into(),
            }),
        }
    }

    fn ruta_archivo(directorio: &Path, clave: &str) -> PathBuf {
        directorio.join(format!("eventos-{clave}.json"))
    }

    fn ahora_ms() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0)
    }

    fn guardar_en_disco(estado: &EstadoInterno) -> Result<(), String> {
        let json = serializar_registro(&estado.registro).map_err(|e| e.to_string())?;
        let ruta = Self::ruta_archivo(&estado.directorio, &estado.clave_cargada);
        fs::write(&ruta, json).map_err(|e| format!("no se pudo guardar {ruta:?}: {e}"))
    }

    fn cargar_desde_disco(directorio: &Path, clave: &str) -> RegistroEventosQc {
        let ruta = Self::ruta_archivo(directorio, clave);
        if let Ok(json) = fs::read_to_string(&ruta) {
            deserializar_registro(&json, clave)
        } else {
            RegistroEventosQc::vacio(clave)
        }
    }

    /// Cambia el proyecto activo (guarda el anterior y carga el nuevo).
    pub fn establecer_proyecto(
        &self,
        app: &AppHandle,
        ruta_a: Option<&str>,
        ruta_b: Option<&str>,
    ) -> Result<RegistroEventosQc, String> {
        let clave = RegistroEventosQc::clave_desde_rutas(ruta_a, ruta_b);
        let mut estado = self.inner.lock().map_err(|e| e.to_string())?;

        if estado.clave_cargada != clave {
            Self::guardar_en_disco(&estado)?;
            estado.registro = Self::cargar_desde_disco(&estado.directorio, &clave);
            estado.clave_cargada = clave.clone();
        } else {
            estado.registro.clave_proyecto = clave;
        }

        let copia = estado.registro.clone();
        drop(estado);
        self.emitir_actualizacion(app, &copia)?;
        Ok(copia)
    }

    pub fn listar(
        &self,
        filtro_tipo: Option<TipoEventoQc>,
    ) -> Result<Vec<EventoQc>, String> {
        let estado = self.inner.lock().map_err(|e| e.to_string())?;
        Ok(estado
            .registro
            .listar(filtro_tipo)
            .into_iter()
            .cloned()
            .collect())
    }

    pub fn registro_completo(&self) -> Result<RegistroEventosQc, String> {
        let estado = self.inner.lock().map_err(|e| e.to_string())?;
        Ok(estado.registro.clone())
    }

    pub fn crear_evento(
        &self,
        app: &AppHandle,
        tipo: TipoEventoQc,
        pts_secs: f64,
        titulo: String,
        descripcion: Option<String>,
    ) -> Result<EventoQc, String> {
        let mut estado = self.inner.lock().map_err(|e| e.to_string())?;
        let evento = estado.registro.agregar_evento(
            tipo,
            pts_secs,
            titulo,
            descripcion,
            Self::ahora_ms(),
        );
        Self::guardar_en_disco(&estado)?;
        let reg = estado.registro.clone();
        drop(estado);
        self.emitir_actualizacion(app, &reg)?;
        Ok(evento)
    }

    pub fn crear_nota(
        &self,
        app: &AppHandle,
        evento_id: u64,
        texto: String,
        pts_secs: f64,
    ) -> Result<EventoQc, String> {
        let mut estado = self.inner.lock().map_err(|e| e.to_string())?;
        estado
            .registro
            .agregar_nota(evento_id, texto, pts_secs, Self::ahora_ms())
            .ok_or_else(|| format!("evento {evento_id} no encontrado"))?;
        let evento = estado
            .registro
            .obtener(evento_id)
            .cloned()
            .ok_or_else(|| format!("evento {evento_id} no encontrado"))?;
        Self::guardar_en_disco(&estado)?;
        let reg = estado.registro.clone();
        drop(estado);
        self.emitir_actualizacion(app, &reg)?;
        Ok(evento)
    }

    pub fn eliminar_evento(&self, app: &AppHandle, id: u64) -> Result<bool, String> {
        let mut estado = self.inner.lock().map_err(|e| e.to_string())?;
        let ok = estado.registro.eliminar_evento(id);
        if ok {
            Self::guardar_en_disco(&estado)?;
        }
        let reg = estado.registro.clone();
        drop(estado);
        if ok {
            self.emitir_actualizacion(app, &reg)?;
        }
        Ok(ok)
    }

    #[must_use]
    pub fn pts_de_evento(&self, id: u64) -> Result<Option<f64>, String> {
        let estado = self.inner.lock().map_err(|e| e.to_string())?;
        Ok(estado.registro.obtener(id).map(|e| e.pts_secs))
    }

    fn emitir_actualizacion(&self, app: &AppHandle, reg: &RegistroEventosQc) -> Result<(), String> {
        if let Err(e) = app.emit("eventos-qc-actualizados", reg.clone()) {
            log::warn!("eventos-qc-actualizados emit: {e}");
        }
        Ok(())
    }
}

/// Parsea filtro desde el frontend (`"manual" | "video" | "audio"` o null).
pub fn parsear_filtro_tipo(s: Option<String>) -> Result<Option<TipoEventoQc>, String> {
    match s {
        None => Ok(None),
        Some(v) if v.is_empty() => Ok(None),
        Some(v) => match v.as_str() {
            "manual" => Ok(Some(TipoEventoQc::Manual)),
            "video" => Ok(Some(TipoEventoQc::Video)),
            "audio" => Ok(Some(TipoEventoQc::Audio)),
            _ => Err(format!("tipo de filtro desconocido: {v}")),
        },
    }
}

/// Directorio `…/diffplayerqc/eventos` dentro de datos de usuario.
pub fn directorio_eventos(app: &AppHandle) -> Result<PathBuf, String> {
    let base = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    Ok(base.join("eventos"))
}
